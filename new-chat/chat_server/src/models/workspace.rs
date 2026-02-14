use crate::{AppError, AppState};
use chat_core::{ChatUser, Workspace};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct WorkspaceInvitation {
    pub id: i64,
    pub workspace_id: i64,
    pub invite_code: String,
    pub created_by: i64,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInvitation {
    pub expires_in_days: Option<i32>,
    pub max_uses: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JoinWorkspace {
    pub invite_code: String,
}


impl AppState {
    pub async fn create_workspace(&self, name: &str, user_id: u64) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(
            "
            INSERT INTO workspaces (name, owner_id)
            VALUES ($1, $2)
            RETURNING id, name, owner_id, created_at
            ",
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(&self.pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_workspace_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            "
            SELECT id, name, owner_id, created_at
            FROM workspaces
            WHERE name = $1
            ",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    pub async fn update_workspace_owner(
        &self,
        owner_id: u64,
        id: u64,
    ) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(
            "
            UPDATE workspaces
            SET owner_id = $1
            WHERE id = $2 AND EXISTS (
                SELECT 1 FROM users WHERE id = $1 AND ws_id = $2
            )
            RETURNING id, name, owner_id, created_at
            ",
        )
        .bind(owner_id as i64)
        .bind(id as i64)
        .fetch_one(&self.pool)
        .await?;
        Ok(ws)
    }

    #[allow(dead_code)]
    pub async fn find_workspace_by_id(&self, id: u64) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            "
            SELECT id, name, owner_id, created_at
            FROM workspaces
            WHERE id = $1
            ",
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    #[allow(dead_code)]
    pub async fn fetch_chat_users(&self, id: u64) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            "
            SELECT id, fullname, email
            FROM users
            WHERE ws_id = $1 order by id
            ",
        )
        .bind(id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    // Workspace invitation functions
    pub async fn create_invitation(
        &self,
        workspace_id: u64,
        user_id: u64,
        input: &CreateInvitation,
    ) -> Result<WorkspaceInvitation, AppError> {
        // Generate a unique invite code
        let invite_code = generate_invite_code();

        let expires_at = input.expires_in_days.map(|days| {
            chrono::Utc::now() + chrono::Duration::days(days as i64)
        });

        let invitation = sqlx::query_as(
            "
            INSERT INTO workspace_invitations (workspace_id, invite_code, created_by, expires_at, max_uses)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, workspace_id, invite_code, created_by, expires_at, max_uses, used_count, is_active, created_at
            ",
        )
        .bind(workspace_id as i64)
        .bind(&invite_code)
        .bind(user_id as i64)
        .bind(expires_at)
        .bind(input.max_uses)
        .fetch_one(&self.pool)
        .await?;

        Ok(invitation)
    }

    pub async fn get_workspace_invitations(
        &self,
        workspace_id: u64,
    ) -> Result<Vec<WorkspaceInvitation>, AppError> {
        let invitations = sqlx::query_as(
            "
            SELECT id, workspace_id, invite_code, created_by, expires_at, max_uses, used_count, is_active, created_at
            FROM workspace_invitations
            WHERE workspace_id = $1
            ORDER BY created_at DESC
            ",
        )
        .bind(workspace_id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(invitations)
    }

    pub async fn validate_and_use_invitation(
        &self,
        invite_code: &str,
    ) -> Result<WorkspaceInvitation, AppError> {
        // Get the invitation
        let invitation: Option<WorkspaceInvitation> = sqlx::query_as(
            "
            SELECT id, workspace_id, invite_code, created_by, expires_at, max_uses, used_count, is_active, created_at
            FROM workspace_invitations
            WHERE invite_code = $1
            ",
        )
        .bind(invite_code)
        .fetch_optional(&self.pool)
        .await?;

        let invitation = invitation.ok_or_else(|| {
            AppError::NotFound("Invalid invitation code".to_string())
        })?;

        // Check if invitation is active
        if !invitation.is_active {
            return Err(AppError::NotFound("Invitation is no longer active".to_string()));
        }

        // Check if invitation has expired
        if let Some(expires_at) = invitation.expires_at {
            if expires_at < chrono::Utc::now() {
                return Err(AppError::NotFound("Invitation has expired".to_string()));
            }
        }

        // Check if invitation has reached max uses
        if let Some(max_uses) = invitation.max_uses {
            if invitation.used_count >= max_uses {
                return Err(AppError::NotFound("Invitation has reached maximum uses".to_string()));
            }
        }

        // Increment used count
        sqlx::query(
            "
            UPDATE workspace_invitations
            SET used_count = used_count + 1
            WHERE id = $1
            ",
        )
        .bind(invitation.id)
        .execute(&self.pool)
        .await?;

        Ok(invitation)
    }

    pub async fn join_workspace_with_invitation(
        &self,
        user_id: u64,
        invite_code: &str,
    ) -> Result<Workspace, AppError> {
        // Validate and use the invitation
        let invitation = self.validate_and_use_invitation(invite_code).await?;

        // Update user's workspace
        sqlx::query(
            "
            UPDATE users
            SET ws_id = $1
            WHERE id = $2
            ",
        )
        .bind(invitation.workspace_id)
        .bind(user_id as i64)
        .execute(&self.pool)
        .await?;

        // Get the workspace
        let workspace = self
            .find_workspace_by_id(invitation.workspace_id as u64)
            .await?
            .ok_or_else(|| AppError::NotFound("Workspace not found".to_string()))?;

        Ok(workspace)
    }

    pub async fn deactivate_invitation(
        &self,
        invitation_id: u64,
        workspace_id: u64,
    ) -> Result<(), AppError> {
        sqlx::query(
            "
            UPDATE workspace_invitations
            SET is_active = false
            WHERE id = $1 AND workspace_id = $2
            ",
        )
        .bind(invitation_id as i64)
        .bind(workspace_id as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

fn generate_invite_code() -> String {
    // Generate a unique invite code using UUID v7
    let uuid = uuid::Uuid::now_v7();
    // Take alphanumeric characters and convert to uppercase
    uuid.to_string()
        .chars()
        .filter(|c: &char| c.is_alphanumeric())
        .take(12)
        .collect::<String>()
        .to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateUser;
    use anyhow::Result;

    #[tokio::test]
    async fn workspace_create_should_work_and_set_owner() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("test", 0).await?;
        let input = CreateUser::new("TeamMeng", &ws.name, "TeamMeng@123.com", "123456");
        let user = state.create_user(&input).await?;

        assert_eq!(ws.name, "test");
        assert_eq!(user.ws_id, ws.id);

        let ws = state
            .update_workspace_owner(user.id as _, ws.id as _)
            .await?;

        assert_eq!(ws.owner_id, user.id);

        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_find_by_name() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.find_workspace_by_name("acme").await?;

        assert!(ws.is_some());
        assert_eq!(ws.unwrap().name, "acme");

        Ok(())
    }

    #[tokio::test]
    async fn workspace_find_by_email_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.find_workspace_by_name("test").await?;

        assert!(ws.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_all_chat_users() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let users = state.fetch_all_chat_users(1).await?;

        assert_eq!(users.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn create_invitation_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let input = CreateInvitation {
            expires_in_days: Some(7),
            max_uses: Some(10),
        };

        let invitation = state.create_invitation(1, 1, &input).await?;

        assert_eq!(invitation.workspace_id, 1);
        assert_eq!(invitation.created_by, 1);
        assert_eq!(invitation.invite_code.len(), 12);
        assert!(invitation.is_active);
        assert_eq!(invitation.used_count, 0);
        assert!(invitation.max_uses.is_some());
        assert_eq!(invitation.max_uses.unwrap(), 10);
        assert!(invitation.expires_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn create_invitation_without_limits_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let input = CreateInvitation {
            expires_in_days: None,
            max_uses: None,
        };

        let invitation = state.create_invitation(1, 1, &input).await?;

        assert_eq!(invitation.workspace_id, 1);
        assert!(invitation.is_active);
        assert!(invitation.expires_at.is_none());
        assert!(invitation.max_uses.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn list_invitations_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // Create a few invitations
        let input1 = CreateInvitation {
            expires_in_days: Some(7),
            max_uses: Some(10),
        };
        state.create_invitation(1, 1, &input1).await?;

        let input2 = CreateInvitation {
            expires_in_days: None,
            max_uses: None,
        };
        state.create_invitation(1, 1, &input2).await?;

        let invitations = state.get_workspace_invitations(1).await?;

        assert_eq!(invitations.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn validate_and_use_invitation_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let input = CreateInvitation {
            expires_in_days: Some(7),
            max_uses: Some(2),
        };

        let invitation = state.create_invitation(1, 1, &input).await?;
        let invite_code = invitation.invite_code.clone();

        // First use
        let result = state.validate_and_use_invitation(&invite_code).await?;
        assert_eq!(result.workspace_id, 1);

        // Check used count increased
        let invitations = state.get_workspace_invitations(1).await?;
        let updated = invitations.iter().find(|i| i.invite_code == invite_code).unwrap();
        assert_eq!(updated.used_count, 1);

        // Second use
        state.validate_and_use_invitation(&invite_code).await?;

        // Third use should fail (max uses reached)
        let result = state.validate_and_use_invitation(&invite_code).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn validate_invalid_invitation_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let result = state.validate_and_use_invitation("INVALIDCODE").await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn join_workspace_with_invitation_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // Create a new workspace
        let ws = state.create_workspace("test_workspace", 0).await?;

        // Create a user in workspace 1
        let input = CreateUser::new("TestUser", "acme", "testuser@test.com", "123456");
        let user = state.create_user(&input).await?;
        assert_eq!(user.ws_id, 1); // Should be in workspace 1

        // Create invitation for the new workspace
        let invite_input = CreateInvitation {
            expires_in_days: Some(7),
            max_uses: Some(10),
        };
        let invitation = state.create_invitation(ws.id as u64, 1, &invite_input).await?;

        // User joins the new workspace
        let joined_ws = state.join_workspace_with_invitation(user.id as u64, &invitation.invite_code).await?;

        assert_eq!(joined_ws.id, ws.id);
        assert_eq!(joined_ws.name, "test_workspace");

        // Verify user's workspace was updated
        let updated_user = state.find_user_by_id(user.id).await?.unwrap();
        assert_eq!(updated_user.ws_id, ws.id);

        Ok(())
    }

    #[tokio::test]
    async fn deactivate_invitation_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let input = CreateInvitation {
            expires_in_days: Some(7),
            max_uses: Some(10),
        };

        let invitation = state.create_invitation(1, 1, &input).await?;

        // Deactivate the invitation
        state.deactivate_invitation(invitation.id as u64, 1).await?;

        // Try to use the deactivated invitation
        let result = state.validate_and_use_invitation(&invitation.invite_code).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn expired_invitation_should_fail() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        // Create an invitation that expires immediately (0 days)
        let input = CreateInvitation {
            expires_in_days: Some(0),
            max_uses: None,
        };

        let invitation = state.create_invitation(1, 1, &input).await?;

        // Wait a moment to ensure expiration
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Try to use the expired invitation
        let result = state.validate_and_use_invitation(&invitation.invite_code).await;
        assert!(result.is_err());

        Ok(())
    }
}
