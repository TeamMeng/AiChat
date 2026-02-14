use crate::{AppError, AppState};
use chat_core::{Chat, ChatType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, ToSchema, Serialize, Deserialize)]
pub struct CreateChat {
    pub name: Option<String>,
    pub members: Vec<i64>,
    pub public: bool,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct UpdateChat {
    pub name: Option<String>,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct AddMembers {
    pub members: Vec<i64>,
}

impl AppState {
    pub async fn create_chat(
        &self,
        input: &CreateChat,
        user_id: u64,
        ws_id: u64,
    ) -> Result<Chat, AppError> {
        if !input.members.contains(&(user_id as i64)) {
            return Err(AppError::CreateChatError(
                "you must be a member of the chat".to_string(),
            ));
        }

        if let Some(name) = &input.name
            && name.len() < 3
        {
            return Err(AppError::CreateChatError(
                "chat name must have at least 3 characters".to_string(),
            ));
        }

        let len = input.members.len();
        if len < 2 {
            return Err(AppError::CreateChatError(
                "chat must have at lease 2 members".to_string(),
            ));
        }

        if len > 8 && input.name.is_none() {
            return Err(AppError::CreateChatError(
                "group chat with more then 8 members must have a name".to_string(),
            ));
        };

        // verify if all members exists
        let users = self.fetch_chat_user_by_ids(&input.members).await?;
        if users.len() != len {
            return Err(AppError::CreateChatError(
                "some users not exists".to_string(),
            ));
        };

        let chat_type = match (&input.name, len) {
            (None, 2) => ChatType::Single,
            (None, _) => ChatType::Group,
            (Some(_), _) => {
                if input.public {
                    ChatType::PublicChannel
                } else {
                    ChatType::PrivateChannel
                }
            }
        };

        let chat = sqlx::query_as(
            "
            INSERT INTO chats (ws_id ,name, type, members)
            VALUES ($1, $2, $3, $4)
            RETURNING id, ws_id, name, type, members, agents, created_at
            ",
        )
        .bind(ws_id as i64)
        .bind(&input.name)
        .bind(chat_type)
        .bind(&input.members)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("chats_ws_id_name_key") {
                    return AppError::CreateChatError(
                        "a chat with this name already exists in your workspace".to_string(),
                    );
                }
            }
            AppError::SqlxError(e)
        })?;

        Ok(chat)
    }

    pub async fn fetch_all_chats(&self, user_id: u64, ws_id: u64) -> Result<Vec<Chat>, AppError> {
        let chats = sqlx::query_as(
            "
            SELECT id, ws_id, name, type, members, agents, created_at
            FROM chats
            WHERE ws_id = $1 AND $2 = ANY(members)
            ",
        )
        .bind(ws_id as i64)
        .bind(user_id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(chats)
    }

    pub async fn get_chat_by_id(&self, id: u64) -> Result<Option<Chat>, AppError> {
        let chat = sqlx::query_as(
            "
            SELECT id, ws_id, name, type, members, agents, created_at
            FROM chats
            WHERE id = $1
            ",
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;
        Ok(chat)
    }

    pub async fn is_chat_member(&self, chat_id: u64, user_id: u64) -> Result<bool, AppError> {
        let is_member = sqlx::query(
            "
            SELECT 1
            FROM chats
            WHERE id = $1 AND $2 = ANY(members)
            ",
        )
        .bind(chat_id as i64)
        .bind(user_id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(is_member.is_some())
    }

    pub async fn update_chat_name(
        &self,
        chat_id: u64,
        name: &str,
    ) -> Result<Chat, AppError> {
        if name.len() < 3 {
            return Err(AppError::CreateChatError(
                "chat name must have at least 3 characters".to_string(),
            ));
        }

        let chat = sqlx::query_as(
            "
            UPDATE chats
            SET name = $1
            WHERE id = $2
            RETURNING id, ws_id, name, type, members, agents, created_at
            ",
        )
        .bind(name)
        .bind(chat_id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(chat)
    }

    pub async fn add_members_to_chat(
        &self,
        chat_id: u64,
        member_ids: &[i64],
    ) -> Result<Chat, AppError> {
        // Verify if all members exist
        let users = self.fetch_chat_user_by_ids(member_ids).await?;
        if users.len() != member_ids.len() {
            return Err(AppError::CreateChatError(
                "some users do not exist".to_string(),
            ));
        }

        let chat = sqlx::query_as(
            "
            UPDATE chats
            SET members = array(SELECT DISTINCT unnest(members || $1))
            WHERE id = $2
            RETURNING id, ws_id, name, type, members, agents, created_at
            ",
        )
        .bind(member_ids)
        .bind(chat_id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(chat)
    }

    pub async fn remove_member_from_chat(
        &self,
        chat_id: u64,
        user_id: u64,
    ) -> Result<(), AppError> {
        // Remove the member from the chat
        sqlx::query(
            "
            UPDATE chats
            SET members = array_remove(members, $1)
            WHERE id = $2
            ",
        )
        .bind(user_id as i64)
        .bind(chat_id as i64)
        .execute(&self.pool)
        .await?;

        // Check if the chat has no members left
        let members_count: Option<(i32,)> = sqlx::query_as(
            "
            SELECT array_length(members, 1) as count
            FROM chats
            WHERE id = $1
            ",
        )
        .bind(chat_id as i64)
        .fetch_optional(&self.pool)
        .await?;

        // If no members left (or members array is empty), delete the chat
        if members_count.is_none() || members_count.unwrap().0 == 0 {
            self.delete_chat(chat_id).await?;
        }

        Ok(())
    }

    pub async fn delete_chat(&self, chat_id: u64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM chats WHERE id = $1")
            .bind(chat_id as i64)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
impl CreateChat {
    pub fn new(name: &str, members: &[i64], public: bool) -> Self {
        let name = if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        };
        Self {
            name,
            members: members.to_vec(),
            public,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateChat::new("", &[1, 2], false);
        let chat = state.create_chat(&input, 1, 1).await?;

        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 2);
        assert_eq!(chat.r#type, ChatType::Single);

        Ok(())
    }

    #[tokio::test]
    async fn create_public_named_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateChat::new("test", &[1, 2, 3], true);
        let chat = state.create_chat(&input, 1, 1).await?;

        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 3);
        assert_eq!(chat.r#type, ChatType::PublicChannel);

        Ok(())
    }

    #[tokio::test]
    async fn chat_get_by_id_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let chat = state.get_chat_by_id(1).await?.expect("chat should exists");

        assert_eq!(chat.id, 1);
        assert_eq!(chat.name, Some("general".to_string()));
        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn chat_fetch_all_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let chats = state.fetch_all_chats(1, 1).await?;

        assert_eq!(chats.len(), 4);

        Ok(())
    }

    #[tokio::test]
    async fn chat_is_member_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // exist
        let is_member = state.is_chat_member(1, 1).await?;
        assert!(is_member);
        // don't exist
        let is_member = state.is_chat_member(1, 6).await?;
        assert!(!is_member);

        let is_member = state.is_chat_member(10, 1).await?;
        assert!(!is_member);

        let is_member = state.is_chat_member(2, 4).await?;
        assert!(!is_member);

        Ok(())
    }

    #[tokio::test]
    async fn update_chat_name_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let chat = state.update_chat_name(1, "new name").await?;

        assert_eq!(chat.id, 1);
        assert_eq!(chat.name, Some("new name".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn update_chat_name_with_short_name_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let result = state.update_chat_name(1, "ab").await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn add_members_to_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // Chat 2 initially has members [1, 2, 3]
        // Add members 4 and 5 to it
        let chat = state.add_members_to_chat(2, &[4, 5]).await?;

        assert_eq!(chat.id, 2);
        assert!(chat.members.contains(&4));
        assert!(chat.members.contains(&5));
        assert_eq!(chat.members.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn add_duplicate_members_should_not_duplicate() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // Try to add existing member
        let chat = state.add_members_to_chat(1, &[1, 2]).await?;

        assert_eq!(chat.id, 1);
        // Should still have 5 members, not 7
        assert_eq!(chat.members.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn add_nonexistent_members_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let result = state.add_members_to_chat(1, &[999]).await;

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn remove_member_from_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        state.remove_member_from_chat(1, 5).await?;

        let chat = state.get_chat_by_id(1).await?.expect("chat should exist");
        assert!(!chat.members.contains(&5));
        assert_eq!(chat.members.len(), 4);

        Ok(())
    }

    #[tokio::test]
    async fn delete_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // Create a new chat to delete
        let input = CreateChat::new("test chat", &[1, 2], false);
        let chat = state.create_chat(&input, 1, 1).await?;
        let chat_id = chat.id as u64;

        // Delete the chat
        state.delete_chat(chat_id).await?;

        // Verify it's deleted
        let result = state.get_chat_by_id(chat_id).await?;
        assert!(result.is_none());

        Ok(())
    }
}
