CREATE OR REPLACE FUNCTION notify_message_deleted()
  RETURNS TRIGGER AS $$
DECLARE
  USERS bigint[];
BEGIN
  SELECT members INTO USERS FROM chats WHERE id = OLD.chat_id;
  PERFORM pg_notify('message_deleted', json_build_object(
    'message_id', OLD.id,
    'chat_id', OLD.chat_id,
    'members', USERS
  )::text);
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER message_deleted_trigger
  AFTER DELETE ON messages
  FOR EACH ROW
  EXECUTE FUNCTION notify_message_deleted();
