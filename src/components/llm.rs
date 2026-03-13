use async_trait::async_trait;

use super::{Conversation, Tool, StreamingMessage};

pub type StreamingCallback = Box<dyn FnMut(StreamingMessage) + Send>;

#[async_trait]
pub trait LLM {
    fn add_tool(&self, tool: Tool);
    async fn complete(&mut self, messages: Conversation, on_streaming_message: StreamingCallback) -> Conversation;
}
