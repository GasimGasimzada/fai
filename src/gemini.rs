use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config;

/// Structure for message parts
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Part {
    text: String,
}

/// Structure for conversation contents
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    role: String,
    parts: Vec<Part>,
}

/// Configuration for text generation
#[derive(Serialize, Deserialize )]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    temperature: f64,
    top_k: i32,
    top_p: f64,
    max_output_tokens: i32,
    response_mime_type: String,
}

/// Full request body
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
    contents: Vec<Content>,
    system_instruction: Content,
    generation_config: GenerationConfig,
}

// Define the structure for the candidate
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    pub content: Content,
}

// Define the structure for the root response
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    candidates: Vec<Candidate>,
}

/// Function to make the request
pub async fn format_text(config: &config::Gemini, input: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = &config.api_key;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let request_body = RequestBody {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: format!("Input: {}, Prompt: {}", input, prompt),
            }],
        }],
        system_instruction: Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: "format `input` using `prompt`".to_string(),
            }],
        },
        generation_config: GenerationConfig {
            temperature: 1.0,
            top_k: 40,
            top_p: 0.95,
            max_output_tokens: 8192,
            response_mime_type: "text/plain".to_string(),
        },
    };

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    if let Some(candidate) = response.candidates.get(0) {
        if let Some(part) = candidate.content.parts.get(0) {
            return Ok(part.text.clone());
        }
    }

    Err("Failed response".into())
}
