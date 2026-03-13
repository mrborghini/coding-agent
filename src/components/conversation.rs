/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<LLMMessage>,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub required: bool,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub param_type: ParameterType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

pub struct CompletionResult {
    pub conversation: Conversation,
    pub tool_calls: Vec<ToolCall>,
}

impl Tool {
    pub fn new<S: AsRef<str>>(name: S, description: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            description: description.as_ref().to_string(),
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter<S: AsRef<str>>(
        &mut self,
        name: S,
        description: S,
        param_type: ParameterType,
        required: bool,
    ) {
        let name = name.as_ref().to_string();
        let description = description.as_ref().to_string();
        self.parameters.push(ToolParameter {
            required,
            name,
            description,
            param_type,
            enum_values: None,
        });
    }

    pub fn add_enum_parameter<S: AsRef<str>>(
        &mut self,
        name: S,
        description: S,
        values: Vec<&str>,
        required: bool,
    ) {
        let name = name.as_ref().to_string();
        let description = description.as_ref().to_string();
        self.parameters.push(ToolParameter {
            required,
            name,
            description,
            param_type: ParameterType::String,
            enum_values: Some(values.into_iter().map(|v| v.to_string()).collect()),
        });
    }
}

impl Conversation {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn add_message(&mut self, message: LLMMessage) {
        self.messages.push(message);
    }
}
