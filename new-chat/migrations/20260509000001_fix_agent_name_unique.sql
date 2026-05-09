-- Fix: agent name should be unique per chat, not globally unique
ALTER TABLE chat_agents DROP CONSTRAINT chat_agents_name_key;
ALTER TABLE chat_agents ADD CONSTRAINT chat_agents_chat_id_name_key UNIQUE (chat_id, name);
