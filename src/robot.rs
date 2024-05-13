// robot.rs
//! bot.rs uses Gemini, robot.rs uses OpenAI

use dotenv::dotenv;
use reqwest::{header, Client, Request, StatusCode};
use serde::{Deserialize, Serialize};
// use std::any::Any;
use std::env;

// Structs for the request body
/*

*/

/// ## Request body 
/// Given a list of messages comprising a conversation, the model will return a response.
/// 
/// `POST https://api.openai.com/v1/chat/completions` creates a model response for the given chat conversation.
/// 🌐 https://platform.openai.com/docs/api-reference/chat/object?lang=curl
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
struct RequestBody { model: String, messages: Vec<Message>}

/// ## Message 🌐 https://platform.openai.com/docs/api-reference/chat/create#chat-create-messages
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
struct Message { role: Role, content: String}

/// ## Role
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
enum Role { User, System, Assistant }

//// ## Model
// #[rustfmt::skip]
// enum Model { Model }

/* Completion body
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "gpt-3.5-turbo-0125",
  "system_fingerprint": "fp_44709d6fcb",
  "choices": [{
    "index": 0,
    "message": {
      "role": "assistant",
      "content": "\n\nHello there, how may I assist you today?",
    },
    "logprobs": null,
    "finish_reason": "stop"
  }],
  "usage": {
    "prompt_tokens": 9,
    "completion_tokens": 12,
    "total_tokens": 21
  }
}
 */

/// ## Chat Completion
/// 🌐: https://platform.openai.com/docs/api-reference/chat/object
/// {

#[derive(Serialize, Deserialize)]
struct Completion {
    id: String,
    object: String,
    created: String,
    // model: Model,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
    usage: Usage,
}

/// ## Choice
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
struct Choice { index: u32, message: Message, role: Role, content: String, longprobs: u32, finish_reason: String}

/// ## Usage
#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
struct Usage { prompt_tokens: u32, completion_tokens: u32, total_tokens: u32}

/// ## `create_completion`
///
///

pub async fn create_completion(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // ... load API Key from .env file
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // ... create client for requests (reqwests?)
    let client = Client::new();

    // ... API endpoint
    let url = format!("https://api.openai.com/v1/chat/completions?key={}", api_key);
}
