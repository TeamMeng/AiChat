use crate::{AppError, AppState, ErrorOutput, events::AnalyticsEventRow, pb::AnalyticsEvent};
use axum::{
    extract::State,
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use axum_extra::protobuf::Protobuf;
use chat_core::User;
use tracing::info;

#[utoipa::path(
    post,
    path = "/api/event",
    responses(
        (status = 200, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput),
        (status = 500, description = "Internal server error", body = ErrorOutput)
    ),
    security(
        ("token"=[])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    info!("received event: {:?}", event);
    let mut row = AnalyticsEventRow::try_from(event)?;

    if let Some(user) = parts.extensions.get::<User>() {
        row.user_id = Some(user.id.to_string())
    } else {
        row.user_id = None
    };

    let mut insert = state
        .client
        .insert::<AnalyticsEventRow>("analytics_events")
        .await?;
    insert.write(&row).await?;
    insert.end().await?;

    Ok(StatusCode::CREATED)
}
