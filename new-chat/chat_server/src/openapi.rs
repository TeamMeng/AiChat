use crate::{
    AppState, AuthOutput,
    error::ErrorOutput,
    handlers::*,
    models::{
        ChatFile, CreateAgent, CreateChat, CreateMessage, ListMessages, SigninUser, UpdateAgent,
    },
};
use axum::Router;
use chat_core::{AgentType, Chat, ChatAgent, ChatType, ChatUser, Message, User, Workspace};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub(crate) trait OpenApiRouter {
    fn openapi(self) -> Self;
}

#[derive(OpenApi)]
#[openapi(
    paths(
        signin_handler,
        signup_handler,
        list_chat_handler,
        create_chat_handler,
        get_chat_handler,
        send_message_handler,
        list_chat_users_handler,
        list_message_handler,
        create_agent_handler,
        update_agent_handler,
        list_agent_handler
    ),
    components(schemas(AuthOutput, Chat, ChatType, ChatUser, ChatFile, CreateChat, ChatUser, Message,
         CreateMessage, ListMessages, SigninUser, User, Workspace, ErrorOutput, CreateAgent, UpdateAgent, ChatAgent, AgentType)),
    modifiers(&SecurityAddon),
    tags(
        (name = "Chat", description = "Chat related operations")
    )
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

impl OpenApiRouter for Router<AppState> {
    fn openapi(self) -> Self {
        self.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    }
}
