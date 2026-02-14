mod agent;
mod chat;
mod file;
mod message;
mod user;
mod workspace;

pub use agent::{CreateAgent, UpdateAgent};
pub use chat::{CreateChat, UpdateChat, AddMembers};
pub use message::{CreateMessage, ListMessages};
pub use user::{CreateUser, SigninUser, ChangePasswordInput};
pub use workspace::{WorkspaceInvitation, CreateInvitation, JoinWorkspace};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String, // extract ext from filename or mine type
    pub hash: String,
}
