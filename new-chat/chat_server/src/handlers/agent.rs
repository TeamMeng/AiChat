use crate::{
    AppError, AppState,
    models::{CreateAgent, UpdateAgent},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chat_core::ChatAgent;

/// List all agents in the chat
#[utoipa::path(
    get,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "List of agents", body = Vec<ChatAgent>)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn list_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let agents = state.list_agents(chat_id as _).await?;
    Ok((StatusCode::OK, Json(agents)).into_response())
}

/// Create a new agent in the chat
#[utoipa::path(
    post,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Agent created", body = ChatAgent)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn create_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<CreateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.create_agent(input, chat_id as _).await?;
    Ok((StatusCode::OK, Json(agent)).into_response())
}

/// Update the agent by id
#[utoipa::path(
    patch,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id"),
    ),
    responses(
        (status = 200, description = "Agent updated", body = ChatAgent)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn update_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<UpdateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.update_agent(input, chat_id as _).await?;
    Ok((StatusCode::OK, Json(agent)).into_response())
}
