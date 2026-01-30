use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::warn;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("clickhouse error: {0}")]
    ClickhouseError(#[from] clickhouse::error::Error),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("generic error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("protobuf error: {0}")]
    ResponseError(String),

    #[error("missing event context")]
    MissingEventContext,

    #[error("missing event data")]
    MissingEventData,

    #[error("missing system info")]
    MissingSystemInfo,
}

impl ErrorOutput {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::ClickhouseError(_)
            | Self::SqlxError(_)
            | Self::IoError(_)
            | Self::AnyError(_)
            | Self::ResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MissingEventContext | Self::MissingEventData | Self::MissingSystemInfo => {
                StatusCode::BAD_REQUEST
            }
        };

        let msg = self.to_string();
        warn!("Status: {}, error: {}", status, msg);

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
