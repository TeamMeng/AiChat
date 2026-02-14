use crate::{AppError, AppState, error::ErrorOutput, models::{CreateChat, UpdateChat, AddMembers}};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chat_core::{Chat, User};

/// List all chats in the workspace of the user
#[utoipa::path(
    get,
    path = "/api/chats",
    responses(
        (status = 200, description = "List of chats", body = Vec<Chat>)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chats = state.fetch_all_chats(user.id as _, user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(chats)).into_response())
}

/// Get the chat info by id
#[utoipa::path(
    get,
    path = "/api/chats/{id}",
    params(
        ("id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Get found", body = Chat),
        (status = 400, description = "Get not found", body = ErrorOutput)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn get_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.get_chat_by_id(id).await?;
    match chat {
        Some(chat) => Ok((StatusCode::OK, Json(chat)).into_response()),
        None => Ok((
            StatusCode::NOT_FOUND,
            AppError::NotFound(format!("chat id: {} not found", id)),
        )
            .into_response()),
    }
}

pub(crate) async fn update_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateChat>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is a member of the chat
    if !state.is_chat_member(id, user.id as _).await? {
        return Err(AppError::NotFound(format!("chat id: {} not found", id)));
    }

    if let Some(name) = input.name {
        let chat = state.update_chat_name(id, &name).await?;
        Ok((StatusCode::OK, Json(chat)).into_response())
    } else {
        Err(AppError::CreateChatError("name is required".to_string()))
    }
}

pub(crate) async fn delete_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is a member of the chat
    if !state.is_chat_member(id, user.id as _).await? {
        return Err(AppError::NotFound(format!("chat id: {} not found", id)));
    }

    state.delete_chat(id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub(crate) async fn add_members_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<AddMembers>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is a member of the chat
    if !state.is_chat_member(id, user.id as _).await? {
        return Err(AppError::NotFound(format!("chat id: {} not found", id)));
    }

    let chat = state.add_members_to_chat(id, &input.members).await?;
    Ok((StatusCode::OK, Json(chat)).into_response())
}

pub(crate) async fn remove_member_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((chat_id, member_id)): Path<(u64, u64)>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is a member of the chat
    if !state.is_chat_member(chat_id, user.id as _).await? {
        return Err(AppError::NotFound(format!("chat id: {} not found", chat_id)));
    }

    // User can only remove themselves
    if user.id as u64 != member_id {
        return Err(AppError::NotFound("you can only remove yourself".to_string()));
    }

    state.remove_member_from_chat(chat_id, member_id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

/// Create a new chat in the workspace of the user
#[utoipa::path(
    post,
    path = "/api/chats/",
    responses(
        (status = 201, description = "Chat created", body = Chat)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state
        .create_chat(&input, user.id as _, user.ws_id as _)
        .await?;
    Ok((StatusCode::CREATED, Json(chat)).into_response())
}
