-- ClickHouse Analytics Database Initialization Script
-- Run this script to create the analytics database and tables

-- Create database if not exists
CREATE DATABASE IF NOT EXISTS analytics;

-- Create analytics_events table
CREATE TABLE IF NOT EXISTS analytics.analytics_events
(
    -- EventContext fields
    client_id String,
    app_version String,
    system_os String,
    system_arch String,
    system_locale String,
    system_timezone String,
    user_id Nullable(String),
    ip Nullable(String),
    user_agent Nullable(String),
    geo_country Nullable(String),
    geo_region Nullable(String),
    geo_city Nullable(String),
    client_ts Int64,
    server_ts Int64,
    -- Common fields
    event_type String,
    -- AppExitEvent fields
    exit_code Nullable(String),
    -- UserLoginEvent
    login_email Nullable(String),
    -- UserLogoutEvent
    logout_email Nullable(String),
    -- UserRegisterEvent
    register_email Nullable(String),
    register_workspace_id Nullable(String),
    -- ChatCreatedEvent
    chat_created_workspace_id Nullable(String),
    -- MessageSentEvent
    message_chat_id Nullable(String),
    message_type Nullable(String),
    message_size Nullable(Int32),
    message_total_files Nullable(Int32),
    -- ChatJoinedEvent
    chat_joined_id Nullable(String),
    -- ChatLeftEvent
    chat_left_id Nullable(String),
    -- NavigationEvent
    navigation_from Nullable(String),
    navigation_to Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (client_ts, client_id);
