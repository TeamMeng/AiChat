use crate::AppState;
use anyhow::Result;
use chat_core::{Chat, Message};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;
use std::{collections::HashSet, sync::Arc};
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceDeleted {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceUpdated {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJoinedWorkspace {
    pub workspace_id: i64,
    pub workspace_name: String,
    pub user_id: i64,
    pub user_name: String,
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum AppEvent {
    NewChat(Chat),
    AddToChat(Chat),
    RemoveFromChat(Chat),
    NewMessage(Message),
    WorkspaceDeleted(WorkspaceDeleted),
    WorkspaceUpdated(WorkspaceUpdated),
    UserJoinedWorkspace(UserJoinedWorkspace),
}

#[derive(Debug)]
struct Notification {
    user_ids: HashSet<u64>,
    event: Arc<AppEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatUpdated {
    op: String,
    old: Option<Chat>,
    new: Option<Chat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageCreated {
    message: Message,
    members: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceDeletedPayload {
    workspace: WorkspaceInfo,
    users: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceUpdatedPayload {
    workspace: WorkspaceInfo,
    users: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserJoinedWorkspacePayload {
    workspace_id: i64,
    workspace_name: String,
    user_id: i64,
    user_name: String,
    user_email: String,
    users: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceInfo {
    id: i64,
    name: String,
}

pub async fn setup_pg_listener(state: AppState) -> Result<()> {
    let mut listener = PgListener::connect(&state.config.server.db_url).await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;
    listener.listen("workspace_deleted").await?;
    listener.listen("workspace_updated").await?;
    listener.listen("user_joined_workspace").await?;

    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(Ok(notif)) = stream.next().await {
            info!("Received notification: {:?}", notif);

            let notif = Notification::load(notif.channel(), notif.payload())?;

            let users = &state.users;

            for user_id in notif.user_ids {
                if let Some(tx) = users.get(&user_id)
                    && let Err(e) = tx.send(notif.event.clone())
                {
                    warn!("failed to send notif to user {}: {}", user_id, e);
                }
            }
        }
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

impl Notification {
    fn load(r#type: &str, payload: &str) -> Result<Self> {
        match r#type {
            "chat_updated" => {
                let payload: ChatUpdated = serde_json::from_str(payload)?;
                info!("ChatUpdated: {:?}", payload);
                let user_ids =
                    get_affected_chat_user_ids(payload.old.as_ref(), payload.new.as_ref());
                let event = match payload.op.as_str() {
                    "INSERT" => AppEvent::NewChat(payload.new.expect("new should exist")),
                    "UPDATE" => AppEvent::AddToChat(payload.new.expect("new should exist")),
                    "DELETE" => AppEvent::RemoveFromChat(payload.old.expect("old should exist")),
                    _ => return Err(anyhow::anyhow!("Invalid operation")),
                };
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(payload)?;
                let user_ids = payload.members.iter().map(|v| *v as u64).collect();
                Ok(Self {
                    user_ids,
                    event: Arc::new(AppEvent::NewMessage(payload.message)),
                })
            }
            "workspace_deleted" => {
                let payload: WorkspaceDeletedPayload = serde_json::from_str(payload)?;
                info!("WorkspaceDeleted: {:?}", payload);
                let user_ids = payload.users.iter().map(|v| *v as u64).collect();
                let event = AppEvent::WorkspaceDeleted(WorkspaceDeleted {
                    id: payload.workspace.id,
                    name: payload.workspace.name,
                });
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "workspace_updated" => {
                let payload: WorkspaceUpdatedPayload = serde_json::from_str(payload)?;
                info!("WorkspaceUpdated: {:?}", payload);
                let user_ids = payload.users.iter().map(|v| *v as u64).collect();
                let event = AppEvent::WorkspaceUpdated(WorkspaceUpdated {
                    id: payload.workspace.id,
                    name: payload.workspace.name,
                });
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "user_joined_workspace" => {
                let payload: UserJoinedWorkspacePayload = serde_json::from_str(payload)?;
                info!("UserJoinedWorkspace: {:?}", payload);
                let user_ids = payload.users.iter().map(|v| *v as u64).collect();
                let event = AppEvent::UserJoinedWorkspace(UserJoinedWorkspace {
                    workspace_id: payload.workspace_id,
                    workspace_name: payload.workspace_name,
                    user_id: payload.user_id,
                    user_name: payload.user_name,
                    user_email: payload.user_email,
                });
                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            _ => Err(anyhow::anyhow!("Invalid notification type")),
        }
    }
}

fn get_affected_chat_user_ids(old: Option<&Chat>, new: Option<&Chat>) -> HashSet<u64> {
    match (old, new) {
        (Some(old), Some(new)) => {
            let old_user_ids = old
                .members
                .iter()
                .map(|v| *v as u64)
                .collect::<HashSet<_>>();
            let new_user_ids = new
                .members
                .iter()
                .map(|v| *v as u64)
                .collect::<HashSet<_>>();
            if old_user_ids == new_user_ids {
                HashSet::new()
            } else {
                old_user_ids.union(&new_user_ids).copied().collect()
            }
        }
        (Some(old), None) => old
            .members
            .iter()
            .map(|v| *v as u64)
            .collect::<HashSet<_>>(),

        (None, Some(new)) => new
            .members
            .iter()
            .map(|v| *v as u64)
            .collect::<HashSet<_>>(),
        _ => HashSet::new(),
    }
}
