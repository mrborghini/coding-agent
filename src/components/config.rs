/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

use rust_logger::{Logger, Severity};
use std::env;

pub struct Config {
    pub ollama_url: String,
    pub model: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        let log = Logger::new("Config");
        let ollama_url = env::var("OLLAMA_URL").unwrap_or("http://localhost:11434".to_string());
        let model = env::var("MODEL").unwrap_or(String::new());
        if model.is_empty() {
            log.error(
                "Variable 'MODEL' has not been set in enviroment",
                Severity::Critical,
            );
            return Err("MODEL not set".into());
        }

        Ok(Config {
            ollama_url,
            model,
        })
    }
}
