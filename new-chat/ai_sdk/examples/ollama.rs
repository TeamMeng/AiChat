use ai_sdk::{AiService, Message, OllamaAdapter, Role};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let adapter = OllamaAdapter::default();
    let message = vec![Message {
        role: crate::Role::User,
        content: "Hello".to_string(),
    }];
    let response = adapter.complete(&message).await?;
    println!("response: {}", response);
    Ok(())
}
