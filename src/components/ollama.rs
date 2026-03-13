use super::{
    Config, Conversation, LLM, LLMMessage, Role, StreamingCallback, StreamingMessage, Tool,
};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Response;
use rust_logger::{Logger, Severity};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OllamaStreamMessage {
    role: Role,
    content: String,
    thinking: Option<String>,
}

struct FullContent {
    full_thinking: String,
    full_content: String,
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

#[derive(Serialize, Deserialize)]
pub struct OllamaBody<'a> {
    model: &'a str,
    messages: Vec<LLMMessage>,
    stream: bool,
    options: Option<OllamaOptions>
}

pub struct Ollama<'a> {
    log: Logger,
    cfg: &'a Config,
    http_client: reqwest::Client,
}

impl<'a> Ollama<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        let log = Logger::new("Ollama");
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3600))
            .build()
            .expect("A new http client");
        return Self {
            log,
            cfg,
            http_client,
        };
    }

    async fn handle_streaming(
        &mut self,
        response: Response,
        mut on_streaming_message: StreamingCallback,
    ) -> FullContent {
        // We'll collect the full response here to update the conversation history
        let mut full_content = String::new();
        let mut full_thinking = String::new();

        let mut stream = response.bytes_stream();
        let mut buffer = String::with_capacity(1024);

        while let Some(Ok(chunk)) = stream.next().await {
            if let Ok(s) = std::str::from_utf8(&chunk) {
                buffer.push_str(s);

                while let Some(pos) = buffer.find('\n') {
                    let line = &buffer[..pos]; // Optimized: borrow the line

                    if let Ok(parsed) = serde_json::from_str::<OllamaStreamResponse>(line) {
                        let content = &parsed.message.content;
                        let thinking = parsed.message.thinking.as_deref().unwrap_or("");

                        // 1. Build the message for the callback
                        let msg = StreamingMessage {
                            role: parsed.message.role.clone(),
                            content: content.to_string(),
                            thinking: thinking.to_string(),
                        };

                        // 2. Accumulate for history
                        full_content.push_str(content);
                        full_thinking.push_str(thinking);

                        // 3. PASS THROUGH to the callback
                        on_streaming_message(msg);
                    }

                    buffer.drain(..=pos); // Remove processed data
                }
            }
        }
        return FullContent {
            full_content,
            full_thinking,
        };
    }
}

#[async_trait]
impl LLM for Ollama<'_> {
    fn add_tool(&self, tool: Tool) {
        todo!()
    }

    async fn complete(
        &mut self,
        mut conversation: Conversation,
        on_streaming_message: StreamingCallback,
    ) -> Conversation {
        let ollama_url = format!("{}/api/chat", self.cfg.ollama_url);
        
        let options = OllamaOptions {
            num_ctx: Some(262144),
            temperature: Some(0.6),
            repeat_penalty: Some(1.15),
            min_p: Some(0.05),
            frequency_penalty: Some(0.5),
            ..Default::default()
        };

        let json = OllamaBody {
            model: &self.cfg.model,
            messages: conversation.messages.clone(),
            stream: true,
            options: Some(options)
        };

        let resp = self.http_client.post(ollama_url).json(&json).send().await;
        
        let mut full_content = String::new();

        if let Ok(r) = resp {
            let content = self.handle_streaming(r, on_streaming_message).await;
            full_content = content.full_content;
        }

        // Final step: Update conversation history so the next turn has context
        conversation.messages.push(LLMMessage {
            role: Role::Assistant,
            content: full_content,
            // Add thinking here if your LLMMessage supports it
        });

        conversation
    }
}
