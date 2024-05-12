// decoder.rs
use serde::Deserialize;

// --> Structs for the Response Payload body
// Docstrings are in bot.rs
/* The Response body is an array of "candidates"
{ "candidates": [{
    "content": { "parts": [{"text": ""}], "role": "" },
    "finishReason": "",
    "index": _,
    "safetyRatings": [ {"category": "","probability": ""}, ]
    ]},
  "promptFeedback": {
    "safetyRatings": [ {"category": "", "probability": ""}, ]
  }
}
 */
// 1. Payload
#[rustfmt::skip]
#[derive(Deserialize, Debug)]
struct ResponsePayload { candidates: Vec<Candidate>, }

// 2. Candidate (🌐: https://ai.google.dev/api/rest/v1beta/Candidate)
#[rustfmt::skip]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Candidate { content: Content, safetyRatings: Vec<SafetyRating>, finishReason: String, index: u32, }

// 3. Content
#[rustfmt::skip]
#[derive(Deserialize, Debug)]
struct Content { parts: Vec<Part>, role: String, }

// 4. Part
#[rustfmt::skip]
#[derive(Deserialize, Debug)]
struct Part { text: String,}

// 5. SafetyRating
#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct SafetyRating { category: String, probability: String, }

/// ## `parse_response`
///
/// Parses the JSON response from the Gemini Pro API and extracts the generated text completion.
///
/// This function takes the raw JSON string returned by the Gemini Pro API, deserializes it into Rust structures, and extracts the text output from the first valid candidate. It also performs error handling to ensure the response is correctly formatted and safe.
///
/// ### Arguments
///
/// - `json_string` (`&str`): The raw JSON string received from the Gemini Pro API.
///
/// ### Returns
///
/// A `Result` containing either:
/// - `Ok(String)`: The extracted text completion if parsing and validation are successful.
/// - `Err(Box<dyn std::error::Error>)`: An error message if:
///   - The JSON string is invalid.
///   - No candidates are found in the response.
///   - The candidate's role is not "model".
///   - The candidate's output is empty.
///   - The generated content raises safety concerns.
///
/// ### Example
///
/// ```rust
/// let response_json = r#"{"candidates":[{"output":{"parts":[{"text":"This is the generated text."}],"role":"model"},"finishReason":"STOP_REASON_COMPLETED_SUFFICIENTLY","index":0,"safetyRatings":[{"category":"HARM_CATEGORY_DEROGATORY","probability":"NEGLIGIBLE"}]}]}"#;
/// let parsed_text = parse_response(response_json);
///
/// match parsed_text {
///     Ok(text) => println!("Parsed Text: {}", text),
///     Err(err) => eprintln!("Error parsing response: {}", err),
/// }
/// ```

pub fn parse_response(json_string: &str) -> Result<String, Box<dyn std::error::Error>> {
    // ... parse json string
    let response_payload: ResponsePayload = serde_json::from_str(json_string)?;

    // ... extract the candidate
    let candidate = response_payload
        .candidates
        .first()
        .ok_or("No candidates found in response")?;

    // ... the role should be "model"
    if candidate.content.role != "model" {
        return Err("Invalid candidate role".into());
    }

    // ... extract the "text" (completion)
    let content_text = candidate
        .content
        .parts
        .iter()
        .map(|part| part.text.clone())
        .collect::<Vec<String>>()
        .join("\n"); // Combine multiple parts into a single string

    Ok(content_text)
}
