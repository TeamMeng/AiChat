-- Add trigger to notify when a user joins a workspace
CREATE OR REPLACE FUNCTION notify_user_joined_workspace()
RETURNS TRIGGER AS $$
DECLARE
    old_workspace_id BIGINT;
    new_workspace_id BIGINT;
    workspace_name TEXT;
    user_name TEXT;
    user_email TEXT;
    workspace_users BIGINT[];
BEGIN
    -- Only trigger if workspace_id actually changed
    IF OLD.ws_id IS DISTINCT FROM NEW.ws_id THEN
        old_workspace_id := OLD.ws_id;
        new_workspace_id := NEW.ws_id;

        -- Get workspace name
        SELECT name INTO workspace_name
        FROM workspaces
        WHERE id = new_workspace_id;

        -- Get user info
        user_name := NEW.fullname;
        user_email := NEW.email;

        -- Get all users in the new workspace
        SELECT ARRAY_AGG(id)
        INTO workspace_users
        FROM users
        WHERE ws_id = new_workspace_id;

        -- Send notification
        PERFORM pg_notify(
            'user_joined_workspace',
            json_build_object(
                'workspace_id', new_workspace_id,
                'workspace_name', workspace_name,
                'user_id', NEW.id,
                'user_name', user_name,
                'user_email', user_email,
                'users', workspace_users
            )::text
        );
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger on users table
DROP TRIGGER IF EXISTS user_joined_workspace_trigger ON users;
CREATE TRIGGER user_joined_workspace_trigger
    AFTER UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION notify_user_joined_workspace();
