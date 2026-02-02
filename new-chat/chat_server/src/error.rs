use axum::{Json, http::StatusCode, response::IntoResponse};
use chat_core::AgentError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("email already exists: {0}")]
    EmailAleardyExists(String),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("create agent error: {0}")]
    CreateAgentError(String),

    #[error("user {user_id} is not member of chat {chat_id}")]
    NotChatMemberError { user_id: u64, chat_id: u64 },

    #[error("update agent error: {0}")]
    UpdateAgentError(String),

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("not logged in")]
    NotLoggedInError,

    #[error("{0}")]
    ChatFileError(String),

    #[error("not found error: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2 password hash error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("ai agent error: {0}")]
    AiAgentError(#[from] AgentError),
}

impl ErrorOutput {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::EmailAleardyExists(_) => StatusCode::CONFLICT,
            Self::CreateChatError(_)
            | Self::CreateMessageError(_)
            | Self::ChatFileError(_)
            | Self::CreateAgentError(_)
            | Self::UpdateAgentError(_) => StatusCode::BAD_REQUEST,
            Self::NotChatMemberError { .. } => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Argon2Error(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::AiAgentError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotLoggedInError => StatusCode::UNAUTHORIZED,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
