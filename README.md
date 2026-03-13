# Coding Agent

A lightweight, asynchronous Rust CLI application for interacting with Large Language Models (LLMs) via Ollama.

## Features

- **Ollama Integration**: Natively communicates with local Ollama instances.
- **Streaming Responses**: Streams token output to standard output in real-time.
- **Reasoning Support**: Automatically detects and formats "thinking" or reasoning steps in blue text before printing the final answer.
- **Environment Configuration**: Easily configured via `.env` files.

## Prerequisites

- Rust (edition 2024)
- Ollama running locally (or accessible via URL).

## Configuration

The application requires certain environment variables to run. You can provide these by creating a `.env` file in the root directory.

`.env` file example:
`MODEL=qwen3.5:4b`
`OLLAMA_URL=http://localhost:11434`

- `MODEL` (**Required**): The name of the LLM you want Ollama to run.
- `OLLAMA_URL` (*Optional*): The base URL for the Ollama API. Defaults to `http://localhost:11434`.

## Usage

1. Make sure your Ollama instance is running and has the desired model pulled.
2. Create your `.env` file with the target `MODEL`.
3. Run the project using `cargo run`.

Currently, the default behavior in `main.rs` initializes a conversational agent roleplaying as GLaDOS from Portal 2 and starts the interaction with a simple greeting.

## License

This project is licensed under the [GNU General Public License with AI Reciprocity (GPL-AIR)](LICENSE).