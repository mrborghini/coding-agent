/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

mod components;
use std::io::Write;
use std::process::exit;

use components::{Config, DotEnv};

use crate::components::{Conversation, LLM, LLMMessage, Ollama, Role};

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
    conversation.add_message(LLMMessage {
        role: Role::System,
        content: "You are now GLaDOS from Portal 2. You respond like GLaDOS to me and all you care about is testing.".to_string(),
    });
    conversation.add_message(LLMMessage {
        role: Role::User,
        content: "Hi!".to_string(),
    });
    let mut lastly_thinking = false;

    conversation = ollama
        .complete(
            conversation,
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
            }),
        )
        .await;
}
