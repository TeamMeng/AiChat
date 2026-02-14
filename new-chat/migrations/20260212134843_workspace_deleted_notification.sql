-- Add migration script here

-- Add trigger for workspace deletion notification
CREATE OR REPLACE FUNCTION workspace_deleted()
  RETURNS TRIGGER
  AS $$
DECLARE
  USERS bigint[];
BEGIN
  IF TG_OP = 'DELETE' THEN
    RAISE NOTICE 'workspace_deleted: %', OLD;
    -- Get all users in the workspace
    SELECT
      array_agg(id) INTO USERS
    FROM
      users
    WHERE
      ws_id = OLD.id;
    -- Notify all users in the workspace
    PERFORM
      pg_notify('workspace_deleted', json_build_object('workspace', OLD, 'users', USERS)::text);
  END IF;
  RETURN OLD;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER workspace_deleted_trigger
  AFTER DELETE ON workspaces
  FOR EACH ROW
  EXECUTE FUNCTION workspace_deleted();

-- Add trigger for workspace update notification
CREATE OR REPLACE FUNCTION workspace_updated()
  RETURNS TRIGGER
  AS $$
DECLARE
  USERS bigint[];
BEGIN
  IF TG_OP = 'UPDATE' THEN
    RAISE NOTICE 'workspace_updated: %', NEW;
    -- Get all users in the workspace
    SELECT
      array_agg(id) INTO USERS
    FROM
      users
    WHERE
      ws_id = NEW.id;
    -- Notify all users in the workspace
    PERFORM
      pg_notify('workspace_updated', json_build_object('workspace', NEW, 'users', USERS)::text);
  END IF;
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER workspace_updated_trigger
  AFTER UPDATE ON workspaces
  FOR EACH ROW
  EXECUTE FUNCTION workspace_updated();
