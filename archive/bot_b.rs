// bot.rs
// THIS IS THE CORRECT REQUEST BODY, BUT THE WRONG RESPONSE BODY

use dotenv::dotenv;
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::env;

// ==> STRUCTS for elements of the request body (🌐: https://ai.google.dev/gemini-api/docs/get-started/rest)
// 1. Request Payload
#[derive(Serialize, Deserialize)]
struct RequestPayload {
    contents: Vec<Content>, // prompt: Prompt,
                            // temperature: f32,
                            // top_k: u32,
                            // top_p: f32,
                            // candidate_count: u32,
                            // max_output_tokens: u32,
                            // stop_sequences: Vec<String>,
                            // safety_settings: Vec<SafetySetting>,
}

// 2. Candidate
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Candidate {
    output: Content,
    // finishReason: String,
    // index: u32,
    safetyRatings: Vec<SafetyRating>, // ignoring snake_case,needs to match API's field name
}

// 3. Content
#[derive(Serialize, Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
    // role: String, // removed
}
// 4. Part and Prompt
#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    text: String,
}

// 5. Response Payload
#[derive(Deserialize)]
struct ResponsePayload {
    candidates: Vec<Candidate>,
}

// 6. Safety Rating
#[derive(Deserialize)]
struct SafetyRating {
    category: String,
    probability: String,
}

// 7. Safety Setting
#[derive(Serialize, Deserialize, Debug)]
struct SafetySetting {
    category: String,
    threshold: u32,
}
pub async fn generate_completion(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // ... load API key from .env file
    dotenv().ok();
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

    // ... create client
    let client = Client::new();

    // ... endpoint URL (🌐: https://ai.google.dev/api/rest/v1beta/corpora/create)
    #[rustfmt::skip]
    // let url = format!("https://generativelanguage.googleapis.com/v1beta2/models/chat-bison-001:generateMessage?key={}", api_key); // text-only model
    #[rustfmt::skip]
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",api_key); // general use model

    // ... post request to API
    // let payload = json!({
    //     "prompt": {
    //         "text": prompt
    //     },
    //     "temperature": 0.7,
    //     "top_k": 40,
    //     "top_p": 0.95,
    //     "candidate_count": 1,
    //     "max_output_tokens": 1024,
    //     "stop_sequences": [],
    //     "safety_settings": [
    //         {"category": "HARM_CATEGORY_DEROGATORY", "threshold": 3},
    //         {"category": "HARM_CATEGORY_TOXICITY","threshold": 3},
    //         {"category": "HARM_CATEGORY_VIOLENCE","threshold": 2},
    //         {"category": "HARM_CATEGORY_SEXUAL","threshold": 3},
    //         {"category": "HARM_CATEGORY_MEDICAL","threshold": 3},
    //         {"category": "HARM_CATEGORY_DANGEROUS","threshold": 3},
    //     ]
    // });
    //-- old payload
    let payload = RequestPayload {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    // let serialized_payload = serde_json::to_string(&payload)?; // Serialize payload

    //--- old api call
    // API Call
    let response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        // .body(serialized_payload) // passing string (new method uses macro)
        .json(&payload)
        .send()
        .await?;

    // ... API call
    // let response = client
    //     .post(url)
    //     .header(header::CONTENT_TYPE, "application/json")
    //     .json(&payload)
    //     .send()
    //     .await?;

    // HTTP Status Check
    let status = response.status();
    if status != StatusCode::OK {
        let error_text = response.text().await?;
        return Err(format!("Request failed: {} - {}", status, error_text).into());
    }

    /* -- old HTTP status check
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    } */

    /* -- request body test
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);
    */

    // Deserialize response
    let response_payload: ResponsePayload = response.json().await?;
    let candidate = response_payload
        .candidates
        .first()
        .ok_or("No candidates found in response")?;

    //Check candidate is correct type and exists
    if let Some(output) = &candidate.output.parts.first() {
        // Safety check
        for rating in &candidate.safetyRatings {
            if rating.probability == "LIKELY" || rating.probability == "VERY_LIKELY" {
                return Err(format!("Potentially harmful content: {}", rating.category).into());
            }
        }
        return Ok(output.text.clone());
    }

    Err("Invalid candidate response".into())
}

/*/
    let response_payload: ResponsePayload = response.json().await?;
    let output = response_payload
        .candidates
        .first() // Get the first (and usually only) candidate
        .ok_or("No candidates found in response")?
        .output
        .clone();

    // Safety check (you might want to customize this)
    for rating in &response_payload.candidates[0].safetyRatings {
        if rating.probability == "LIKELY" || rating.probability == "VERY_LIKELY" {
            return Err(format!("Potentially harmful content: {}", rating.category).into());
        }
    }


    // Ok(output)
    Ok(response_text)
}
*/
