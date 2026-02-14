use crate::{AppError, AppState, error::ErrorOutput, models::{CreateInvitation, JoinWorkspace, WorkspaceInvitation}};
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use chat_core::{ChatUser, User};

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of chats", body = Vec<ChatUser>),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.fetch_all_chat_users(user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(users)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/workspaces/invitations",
    responses(
        (status = 201, description = "Invitation created", body = WorkspaceInvitation),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn create_invitation_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateInvitation>,
) -> Result<impl IntoResponse, AppError> {
    let invitation = state
        .create_invitation(user.ws_id as _, user.id as _, &input)
        .await?;
    Ok((StatusCode::CREATED, Json(invitation)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/workspaces/invitations",
    responses(
        (status = 200, description = "List of invitations", body = Vec<WorkspaceInvitation>),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn list_invitations_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let invitations = state.get_workspace_invitations(user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(invitations)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/workspaces/join",
    responses(
        (status = 200, description = "Successfully joined workspace", body = String),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn join_workspace_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<JoinWorkspace>,
) -> Result<impl IntoResponse, AppError> {
    let workspace = state
        .join_workspace_with_invitation(user.id as _, &input.invite_code)
        .await?;

    // Generate new token with updated workspace
    let mut updated_user = user.clone();
    updated_user.ws_id = workspace.id;
    updated_user.ws_name = workspace.name.clone();
    let token = state.ek.sign(updated_user)?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Successfully joined workspace",
        "workspace": workspace,
        "token": token
    }))).into_response())
}

#[utoipa::path(
    delete,
    path = "/api/workspaces/invitations/{id}",
    responses(
        (status = 200, description = "Invitation deactivated"),
        (status = 400, description = "Invalid input", body = ErrorOutput),
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn deactivate_invitation_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    state.deactivate_invitation(id, user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Invitation deactivated"
    }))).into_response())
}
