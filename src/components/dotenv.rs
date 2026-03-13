/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License with AI Reciprocity
 * as published in this repository.
 */

use std::env;
use std::fs;

use rust_logger::{Logger, Severity};

pub struct DotEnv {}

impl DotEnv {
    pub fn parse_file(file_path: &str) {
        let log = Logger::new("DotEnv");
        let content = fs::read_to_string(file_path).unwrap_or(String::new());

        if content == "" {
            log.warning("'.env' file is empty or does not exist.", Severity::Medium);
        }

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..]
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .unwrap_or(line[eq_pos + 1..].trim())
                    .to_string();

                if !key.is_empty() {
                    unsafe {
                        env::set_var(&key, &value);
                    }
                }
            }
        }
    }
}
