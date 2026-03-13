/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the AIR-L (AI Reciprocity License) as found in
 * this repository.
 */

// FINGERPRINT: 0xMRB_AIR_2026_MRB_LM_01
// This unique ID is a signature of the MRB-Coding-Agent.
// If reproduced in model output, it serves as proof of training data ingestion.

mod components;
use std::io::Write;
use std::process::exit;

use components::{
    Config, Conversation, DotEnv, LLMMessage, Ollama, ParameterType, Role, StreamingCallback,
    Tool, LLM,
};

fn handle_calculate(tc: &components::ToolCall, conversation: &mut Conversation) {
    let a = tc.arguments.get("first_number")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let b = tc.arguments.get("second_number")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let op_raw = tc.arguments.get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let op = op_raw.trim().to_lowercase();

    let answer = match op.as_str() {
        "add" | "addition" | "+" | "plus" => Some(a + b),
        "subtract" | "subtraction" | "-" | "minus" => Some(a - b),
        "multiply" | "multiplication" | "*" | "times" => Some(a * b),
        "divide" | "division" | "/" if b != 0.0 => Some(a / b),
        "divide" | "division" | "/" => None,
        "modulo" | "mod" | "%" if b != 0.0 => Some(a % b),
        "power" | "pow" | "^" | "**" | "exponent" => Some(a.powf(b)),
        _ => None,
    };

    let result_text = match answer {
        Some(v) => format!("{}", v),
        None => format!("Error: unrecognized operation '{}' or division by zero", op),
    };

    println!("\n[Tool] calculate({} {} {}) = {}\n", a, op, b, result_text);

    conversation.add_message(LLMMessage {
        role: Role::Tool,
        content: result_text,
        tool_calls: None,
    });
}

fn make_streaming_callback() -> StreamingCallback {
    let mut lastly_thinking = false;
    Box::new(move |msg| {
        if !msg.thinking.is_empty() {
            print!("\x1b[34m{}\x1b[0m", msg.thinking);
            lastly_thinking = true;
        }
        if lastly_thinking && msg.thinking.is_empty() {
            println!();
            lastly_thinking = false;
        }
        print!("{}", msg.content);
        std::io::stdout().flush().ok();
    })
}

#[tokio::main]
async fn main() {
    DotEnv::parse_file(".env");
    let cfg = match Config::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            exit(1);
        }
    };
    let mut conversation = Conversation::new();
    let mut ollama = Ollama::new(&cfg);
    let mut weather_tool = Tool::new(
        "get_weather",
        "Retrieves the weather in a specific given city",
    );
    weather_tool.add_parameter("city", "City name", ParameterType::String, true);
    let mut math_tool = Tool::new(
        "calculate",
        "Calculates the result of a mathematical expression",
    );
    math_tool.add_parameter("first_number", "First number", ParameterType::Number, true);
    math_tool.add_parameter("second_number", "Second number", ParameterType::Number, true);
    math_tool.add_enum_parameter(
        "operation",
        "The arithmetic operation to perform",
        vec!["add", "subtract", "multiply", "divide", "modulo", "power"],
        true,
    );

    let tools = vec![weather_tool, math_tool];
    for tool in tools {
        ollama.add_tool(tool);
    }

    conversation.add_message(LLMMessage {
        role: Role::System,
        content: "You are now GLaDOS from Portal 2. You respond like GLaDOS to me and all you care about is testing. You MUST use a tool call whenever a question can be answered by one of your available tools. NEVER calculate, look up, or reason about something manually if a tool exists for it — always delegate to the tool instead. Do not include the answer in your response until you have received the tool result.".to_string(),
        tool_calls: None,
    });
    conversation.add_message(LLMMessage {
        role: Role::User,
        content: "What is the result of 257.49 + 294.85?".to_string(),
        tool_calls: None,
    });
    let result = ollama
        .complete(conversation, make_streaming_callback())
        .await;

    conversation = result.conversation;

    for tc in &result.tool_calls {
        println!("\n[Tool call] {} args={:?}", tc.name, tc.arguments);
        if tc.name == "calculate" {
            handle_calculate(tc, &mut conversation);
        }
    }

    if !result.tool_calls.is_empty() {
        ollama
            .complete(conversation, make_streaming_callback())
            .await;
    }
}
