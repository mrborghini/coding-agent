use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

#[derive(Serialize, Deserialize)]
pub struct StreamingMessage {
   pub role: Role,
   pub content: String,
   pub thinking: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<LLMMessage>,
}

#[derive(Serialize, Deserialize)]
pub enum ParameterType {
    #[serde(rename = "string")]
    String,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "integer")]
    Integer,
}

#[derive(Serialize, Deserialize)]
pub struct ToolParameter {
    required: bool,
    name: String,
    description: String,
    #[serde(rename = "type")]
    param_type: ParameterType,
}

#[derive(Serialize, Deserialize)]
pub struct Tool {
    name: String,
    description: String,
    parameters: Vec<ToolParameter>,
}

impl Conversation {
    pub fn new() -> Self {
        let messages: Vec<LLMMessage> = Vec::new();
        Self { messages }
    }

    pub fn add_message(&mut self, message: LLMMessage) {
        self.messages.push(message);
    }

    pub fn get_token_count(&self) -> usize {
        let copy = &self.messages;
        let mut total_tokens = 0;
        for msg in copy {
            total_tokens += (msg.content.len() + 3) / 4;
        }
        total_tokens
    }
    
    pub fn get_message_count(&self) -> usize {
        self.messages.len()
    }
    
    pub fn get_message_string(&self) -> String {
        let mut buffer = String::new();
        let copy = &self.messages;
        for msg in copy {
            buffer.push_str(&msg.content);
        }
        buffer
    }
}
