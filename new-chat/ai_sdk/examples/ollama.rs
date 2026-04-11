use ai_sdk::{AiService, Message, OllamaAdapter};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let adapter = OllamaAdapter::default();
    // 这里对应 chat_server 当前常见的单轮调用方式。
    let message = vec![Message::user("Hello")];
    let response = adapter.complete(&message).await?;
    println!("response: {}", response);
    Ok(())
}
