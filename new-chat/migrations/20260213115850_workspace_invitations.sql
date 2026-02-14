-- Add migration script here

-- Create workspace invitations table
CREATE TABLE IF NOT EXISTS workspace_invitations (
    id BIGSERIAL PRIMARY KEY,
    workspace_id BIGINT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    invite_code VARCHAR(32) NOT NULL UNIQUE,
    created_by BIGINT NOT NULL REFERENCES users(id),
    expires_at TIMESTAMPTZ,
    max_uses INTEGER,
    used_count INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS workspace_invitations_code_index ON workspace_invitations(invite_code);
CREATE INDEX IF NOT EXISTS workspace_invitations_workspace_id_index ON workspace_invitations(workspace_id);
