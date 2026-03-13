/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

use async_trait::async_trait;

use super::{CompletionResult, Conversation, Tool, StreamingMessage};

pub type StreamingCallback = Box<dyn FnMut(StreamingMessage) + Send>;

#[async_trait]
pub trait LLM {
    fn add_tool(&mut self, tool: Tool);
    async fn complete(&mut self, messages: Conversation, on_streaming_message: StreamingCallback) -> CompletionResult;
}
