use crate::{
    AppError, AppState,
    error::ErrorOutput,
    models::{CreateUser, SigninUser, ChangePasswordInput},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse, Extension};
use chat_core::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Deserialize)]
pub struct AuthOutput {
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/signup",
    responses(
        (status = 201, description = "User created", body = AuthOutput)
    )
)]
/// Create a new user in the chat system with email, password, workspace, and fullname.
///
/// - If the email already exists, it will return 409.
/// - Otherwise, it will return 201 with a token.
/// - If the workspace doesn't exist, it will create one.
pub(crate) async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(&input).await?;
    let token = state.ek.sign(user)?;
    let body = Json(AuthOutput::new(&token));
    Ok((StatusCode::CREATED, body))
}

/// Sign in a user with email and password
#[utoipa::path(
    post,
    path = "/api/signin",
    responses(
        (status = 200, description = "User signed in", body = AuthOutput)
    )
)]
/// User signin
pub(crate) async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.verify_user(&input).await?;
    match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            let body = Json(AuthOutput::new(&token));
            Ok((StatusCode::OK, body).into_response())
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid email or password".to_string()));
            Ok((StatusCode::FORBIDDEN, body).into_response())
        }
    }
}

/// Change user password
#[utoipa::path(
    post,
    path = "/api/change-password",
    responses(
        (status = 200, description = "Password changed successfully")
    )
)]
/// Change user password - requires authentication
pub(crate) async fn change_password_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<ChangePasswordInput>,
) -> Result<impl IntoResponse, AppError> {
    state.change_password(user.id, &input).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({"message": "Password changed successfully"}))))
}

impl AuthOutput {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn signup_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let fullname = "TeamMeng";
        let email = "TeamMeng@123.com";
        let password = "123456";

        let input = CreateUser::new(fullname, "none", email, password);

        let ret = signup_handler(State(state), Json(input))
            .await?
            .into_response();

        assert_eq!(ret.status(), StatusCode::CREATED);

        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");

        Ok(())
    }

    #[tokio::test]
    async fn signup_duplicate_user_should_409() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let fullname = "TeamTest";
        let email = "Test@123.com";
        let password = "123456";
        let workspace = "acme";

        let input = CreateUser::new(fullname, workspace, email, password);

        let ret = signup_handler(State(state), Json(input.clone()))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, format!("email already exists: {}", email));

        Ok(())
    }

    #[tokio::test]
    async fn signin_handler_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Test@123.com";
        let password = "123456";

        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await?
            .into_response();

        assert_eq!(ret.status(), StatusCode::OK);

        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");

        Ok(())
    }

    #[tokio::test]
    async fn signin_with_non_exist_user_should_403() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "TeamMeng@123.com";
        let password = "123456";

        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::FORBIDDEN);

        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "Invalid email or password");

        Ok(())
    }

    #[tokio::test]
    async fn change_password_handler_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // First, sign in to get a user
        let email = "Test@123.com";
        let old_password = "123456";
        let new_password = "newpassword123";

        let signin_input = SigninUser::new(email, old_password);
        let signin_response = signin_handler(State(state.clone()), Json(signin_input))
            .await?
            .into_response();

        assert_eq!(signin_response.status(), StatusCode::OK);

        let body = signin_response.into_body().collect().await?.to_bytes();
        let auth_output: AuthOutput = serde_json::from_slice(&body)?;
        let user = state.dk.verify(&auth_output.token)?;

        // Change password
        let change_input = ChangePasswordInput {
            old_password: old_password.to_string(),
            new_password: new_password.to_string(),
        };

        let ret = change_password_handler(
            Extension(user),
            State(state.clone()),
            Json(change_input),
        )
        .await?
        .into_response();

        assert_eq!(ret.status(), StatusCode::OK);

        // Verify old password no longer works
        let signin_input = SigninUser::new(email, old_password);
        let ret = signin_handler(State(state.clone()), Json(signin_input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::FORBIDDEN);

        // Verify new password works
        let signin_input = SigninUser::new(email, new_password);
        let ret = signin_handler(State(state), Json(signin_input))
            .await?
            .into_response();

        assert_eq!(ret.status(), StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn change_password_with_wrong_old_password_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // Get user
        let user = state
            .find_user_by_id(1)
            .await?
            .expect("user 1 should exist");

        // Try to change password with wrong old password
        let change_input = ChangePasswordInput {
            old_password: "wrongpassword".to_string(),
            new_password: "newpassword123".to_string(),
        };

        let ret = change_password_handler(Extension(user), State(state), Json(change_input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::NOT_FOUND);

        Ok(())
    }
}
