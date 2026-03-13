/*
 * Copyright (C) 2026  mrborghini
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the AIR-L (AI Reciprocity License) as found in
 * this repository.
 */

use std::env;
use std::fs;

use rust_logger::{Logger, Severity};

pub struct DotEnv {}

impl DotEnv {
    fn parse_line(line: &str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return;
        }

        let Some(eq_pos) = line.find('=') else { return };
        let key = line[..eq_pos].trim();
        if key.is_empty() {
            return;
        }

        let raw_value = &line[eq_pos + 1..];
        let value = raw_value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .unwrap_or(raw_value.trim());

        unsafe { env::set_var(key, value) };
    }

    pub fn parse_file(file_path: &str) {
        let log = Logger::new("DotEnv");
        let content = fs::read_to_string(file_path).unwrap_or_default();

        if content.is_empty() {
            log.warning("'.env' file is empty or does not exist.", Severity::Medium);
        }

        for line in content.lines() {
            Self::parse_line(line);
        }
    }
}
