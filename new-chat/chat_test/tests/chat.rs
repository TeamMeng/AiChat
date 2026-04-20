use anyhow::Result;
use chat_core::{Chat, ChatAgent, ChatType, Message};
use chat_server::{AppState, get_router};
use futures::StreamExt;
use reqwest::{
    Client, Response, StatusCode,
    multipart::{Form, Part},
};
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, time::sleep};

const WILD_ADDR: &str = "127.0.0.1:0";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthToken {
    access_token: String,
}

struct NotifyServer;

struct ChatServer {
    addr: SocketAddr,
    token: String,
    client: Client,
}

#[tokio::test]
async fn chat_server_should_work() -> Result<()> {
    let (tdb, state) = AppState::new_for_test().await?;
    let db_url = tdb.url();
    let chat_server = ChatServer::new(state).await?;
    NotifyServer::new(db_url, &chat_server.token).await?;
    let chat = chat_server.create_chat().await?;
    chat_server.create_message(chat.id as _).await?;
    chat_server.upload().await?;
    chat_server.create_agent(chat.id as u64).await?;
    sleep(Duration::from_secs(1)).await;
    Ok(())
}

impl NotifyServer {
    async fn new(db_url: String, token: &str) -> Result<Self> {
        let mut config = notify_server::AppConfig::load()?;
        config.server.db_url = db_url;
        let app = notify_server::get_router(config).await?;
        let listener = TcpListener::bind(WILD_ADDR).await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let response = Client::new()
            .get(format!("http://{}/events?token={}", addr, token))
            .send()
            .await?;

        tokio::spawn(async move {
            if let Err(e) = consume_events(response).await {
                println!("Error: {e}");
            }
        });

        Ok(NotifyServer)
    }
}

async fn consume_events(response: Response) -> Result<()> {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.push_str(std::str::from_utf8(&chunk)?);

        while let Some(idx) = buffer.find("\n\n") {
            let frame = buffer[..idx].replace("\r", "");
            buffer.drain(..idx + 2);

            if frame.trim().is_empty() {
                continue;
            }

            handle_event(&frame)?;
        }
    }

    Ok(())
}

fn handle_event(frame: &str) -> Result<()> {
    let mut event = None;
    let mut data = Vec::new();

    for line in frame.lines() {
        if let Some(value) = line.strip_prefix("event:") {
            event = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data:") {
            data.push(value.trim_start().to_string());
        }
    }

    let data = data.join("\n");

    match event.as_deref() {
        Some("NewChat") => {
            let chat: Chat = serde_json::from_str(&data)?;
            assert_eq!(chat.name, Some("test".to_string()));
            assert_eq!(chat.members, vec![1, 2]);
            assert_eq!(chat.r#type, ChatType::PrivateChannel);
        }
        Some("NewMessage") => {
            let msg: Message = serde_json::from_str(&data)?;
            assert_eq!(msg.content, "Hello World!");
            assert_eq!(msg.files.len(), 0);
            assert_eq!(msg.sender_id, 1);
        }
        Some(other) => {
            panic!("unexpected event: {other} with data: {data}");
        }
        None => {
            if !data.is_empty() {
                panic!("unexpected unnamed event with data: {data}");
            }
        }
    }

    Ok(())
}

impl ChatServer {
    async fn new(state: AppState) -> Result<Self> {
        let app = get_router(state).await?;
        let listener = TcpListener::bind(WILD_ADDR).await?;

        let addr = listener.local_addr()?;

        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let client = Client::new();

        let mut ret = Self {
            addr,
            token: "".to_string(),
            client,
        };

        ret.token = ret.signin().await?;

        Ok(ret)
    }

    async fn signin(&self) -> Result<String> {
        let res = self
            .client
            .post(format!("http://{}/api/signin", self.addr))
            .header("Content-Type", "application/json")
            .body(
                r#"{
                    "email": "Test@123.com",
                    "password": "123456"
                }"#,
            )
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::OK);
        let ret: AuthToken = res.json().await?;
        Ok(ret.access_token)
    }

    async fn create_chat(&self) -> Result<Chat> {
        let res = self
            .client
            .post(format!("http://{}/api/chats", self.addr))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(
                r#"
                {
                    "name": "test",
                    "members": [1, 2],
                    "public": false
                }
            "#,
            )
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::CREATED);
        let chat: Chat = res.json().await?;
        assert_eq!(chat.name, Some("test".to_string()));
        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 2);
        assert_eq!(chat.r#type, ChatType::PrivateChannel);

        Ok(chat)
    }

    async fn create_message(&self, chat_id: u64) -> Result<()> {
        let res = self
            .client
            .post(format!("http://{}/api/chats/{}", self.addr, chat_id))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(
                r#"
                {
                    "content": "Hello World!",
                    "files": []
                }
            "#,
            )
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::CREATED);
        let msg: Message = res.json().await?;
        assert_eq!(msg.content, "Hello World!");
        assert_eq!(msg.sender_id, 1);
        assert_eq!(msg.chat_id, chat_id as i64);
        assert_eq!(msg.files, Vec::<String>::new());

        Ok(())
    }

    async fn upload(&self) -> Result<()> {
        // upload file
        let data = include_bytes!("../Cargo.toml");
        let files = Part::bytes(data)
            .file_name("Cargo.toml")
            .mime_str("text/plain")?;

        let form = Form::new().part("file", files);

        let res = self
            .client
            .post(format!("http://{}/api/upload", self.addr))
            .header("Authorization", format!("Bearer {}", self.token))
            .multipart(form)
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::OK);
        let vec: Vec<String> = res.json().await?;
        assert!(!vec[0].is_empty());

        Ok(())
    }

    async fn create_agent(&self, chat_id: u64) -> Result<ChatAgent> {
        let res = self
            .client
            .post(format!("http://{}/api/chats/{}/agents", self.addr, chat_id))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(
                r#"
                {
                    "name": "test agent",
                    "type": "proxy",
                    "adapter": "test",
                    "model": "llama3.2",
                    "prompt": ""
                }
                "#,
            )
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::OK);
        let agent: ChatAgent = res.json().await?;
        Ok(agent)
    }
}
