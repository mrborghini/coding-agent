/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

use super::{
    CompletionResult, Config, Conversation, LLM, LLMMessage, Role, StreamingCallback,
    StreamingMessage, Tool, ToolCall,
};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct OllamaToolCall {
    id: String,
    function: OllamaToolCallFunction,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaToolCallFunction {
    index: Option<u64>,
    name: String,
    arguments: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaStreamMessage {
    role: Role,
    content: String,
    thinking: Option<String>,
    tool_calls: Option<Vec<OllamaToolCall>>,
}

struct FullContent {
    full_content: String,
    tool_calls: Vec<ToolCall>,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaStreamResponse {
    model: String,
    created_at: String,
    message: OllamaStreamMessage,
    done: bool,
    done_reason: Option<String>,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<u64>,
    eval_count: Option<u64>,
    eval_duration: Option<u64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OllamaOptions {
    pub num_ctx: Option<u32>,
    pub repeat_last_n: Option<i32>,
    pub repeat_penalty: Option<f32>,
    pub temperature: Option<f32>,
    pub seed: Option<u32>,
    pub stop: Option<Vec<String>>,
    pub num_predict: Option<i32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
}

#[derive(Serialize)]
struct OllamaToolProperty {
    #[serde(rename = "type")]
    prop_type: String,
    description: String,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_values: Option<Vec<String>>,
}

#[derive(Serialize)]
struct OllamaToolParameters {
    #[serde(rename = "type")]
    param_type: String,
    required: Vec<String>,
    properties: HashMap<String, OllamaToolProperty>,
}

#[derive(Serialize)]
struct OllamaToolDefFunction {
    name: String,
    description: String,
    parameters: OllamaToolParameters,
}

#[derive(Serialize)]
struct OllamaTool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OllamaToolDefFunction,
}

#[derive(Serialize)]
struct OllamaMessageToolCall {
    function: OllamaMessageToolCallFunction,
}

#[derive(Serialize)]
struct OllamaMessageToolCallFunction {
    name: String,
    arguments: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: Role,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OllamaMessageToolCall>>,
}

impl From<&Tool> for OllamaTool {
    fn from(tool: &Tool) -> Self {
        let mut required = Vec::new();
        let mut properties = HashMap::new();

        for param in &tool.parameters {
            if param.required {
                required.push(param.name.clone());
            }
            properties.insert(
                param.name.clone(),
                OllamaToolProperty {
                    prop_type: serde_json::to_value(&param.param_type)
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    description: param.description.clone(),
                    enum_values: param.enum_values.clone(),
                },
            );
        }

        OllamaTool {
            tool_type: "function".to_string(),
            function: OllamaToolDefFunction {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: OllamaToolParameters {
                    param_type: "object".to_string(),
                    required,
                    properties,
                },
            },
        }
    }
}

impl From<&LLMMessage> for OllamaMessage {
    fn from(msg: &LLMMessage) -> Self {
        let tool_calls = msg.tool_calls.as_ref().map(|calls| {
            calls
                .iter()
                .map(|tc| OllamaMessageToolCall {
                    function: OllamaMessageToolCallFunction {
                        name: tc.name.clone(),
                        arguments: tc.arguments.clone(),
                    },
                })
                .collect()
        });
        OllamaMessage {
            role: msg.role.clone(),
            content: msg.content.clone(),
            tool_calls,
        }
    }
}

#[derive(Serialize)]
pub struct OllamaBody<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OllamaTool>>,
}

pub struct Ollama<'a> {
    cfg: &'a Config,
    http_client: reqwest::Client,
    tools: Vec<Tool>,
}

impl<'a> Ollama<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3600))
            .build()
            .expect("Failed to create http client");
        Self {
            cfg,
            http_client,
            tools: Vec::new(),
        }
    }

    fn process_response(
        parsed: &OllamaStreamResponse,
        full_content: &mut String,
        tool_calls: &mut Vec<ToolCall>,
        on_streaming_message: &mut StreamingCallback,
    ) {
        let content = &parsed.message.content;
        let thinking = parsed.message.thinking.as_deref().unwrap_or("");

        full_content.push_str(content);

        if let Some(calls) = &parsed.message.tool_calls {
            for tc in calls {
                tool_calls.push(ToolCall {
                    name: tc.function.name.clone(),
                    arguments: tc.function.arguments.clone(),
                });
            }
        }

        on_streaming_message(StreamingMessage {
            role: parsed.message.role.clone(),
            content: content.to_string(),
            thinking: thinking.to_string(),
        });
    }

    fn process_buffer(
        buffer: &mut String,
        full_content: &mut String,
        tool_calls: &mut Vec<ToolCall>,
        on_streaming_message: &mut StreamingCallback,
    ) {
        while let Some(pos) = buffer.find('\n') {
            let line = &buffer[..pos];
            if let Ok(parsed) = serde_json::from_str::<OllamaStreamResponse>(line) {
                Self::process_response(&parsed, full_content, tool_calls, on_streaming_message);
            }
            buffer.drain(..=pos);
        }
    }

    fn build_body(&self, conversation: &Conversation) -> OllamaBody<'_> {
        let options = OllamaOptions {
            num_ctx: Some(262144 / 8),
            num_predict: Some(-1),
            temperature: Some(0.6),
            repeat_penalty: Some(1.15),
            min_p: Some(0.05),
            frequency_penalty: Some(0.5),
            ..Default::default()
        };

        let ollama_tools: Vec<OllamaTool> = self.tools.iter().map(OllamaTool::from).collect();
        let tools = if ollama_tools.is_empty() {
            None
        } else {
            Some(ollama_tools)
        };

        OllamaBody {
            model: &self.cfg.model,
            messages: conversation.messages.iter().map(OllamaMessage::from).collect(),
            stream: true,
            options: Some(options),
            tools,
        }
    }

    async fn handle_streaming(
        &mut self,
        response: Response,
        mut on_streaming_message: StreamingCallback,
    ) -> FullContent {
        let mut full_content = String::new();
        let mut tool_calls: Vec<ToolCall> = Vec::new();
        let mut stream = response.bytes_stream();
        let mut buffer = String::with_capacity(1024);

        while let Some(Ok(chunk)) = stream.next().await {
            let Ok(s) = std::str::from_utf8(&chunk) else {
                continue;
            };
            buffer.push_str(s);
            Self::process_buffer(
                &mut buffer,
                &mut full_content,
                &mut tool_calls,
                &mut on_streaming_message,
            );
        }

        FullContent {
            full_content,
            tool_calls,
        }
    }
}

#[async_trait]
impl LLM for Ollama<'_> {
    fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    async fn complete(
        &mut self,
        mut conversation: Conversation,
        on_streaming_message: StreamingCallback,
    ) -> CompletionResult {
        let url = format!("{}/api/chat", self.cfg.ollama_url);
        let body = self.build_body(&conversation);
        let resp = self.http_client.post(url).json(&body).send().await;

        let (full_content, tool_calls) = match resp {
            Ok(r) => {
                let result = self.handle_streaming(r, on_streaming_message).await;
                (result.full_content, result.tool_calls)
            }
            Err(_) => (String::new(), Vec::new()),
        };

        conversation.messages.push(LLMMessage {
            role: Role::Assistant,
            content: full_content,
            tool_calls: if tool_calls.is_empty() {
                None
            } else {
                Some(tool_calls.clone())
            },
        });

        CompletionResult {
            conversation,
            tool_calls,
        }
    }
}
