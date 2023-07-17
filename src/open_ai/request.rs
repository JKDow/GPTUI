use crate::open_ai::objects::{OaiPayload, OaiResponse, OaiMsg, Role};
use crate::open_ai::stream_types::OaiStreamResponse;
use reqwest::{header::{HeaderMap, CONTENT_TYPE}, Client};
use std::env;
use std::io::{Write, stdout};
use futures_util::StreamExt;

pub async fn send_request(payload: OaiPayload) -> Result<OaiResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let uri = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    let client = Client::new();
    println!("Sending request: {:#?}", payload);
    let response = client
        .post(uri)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    println!("Response: {:#?}", response);
    let response = response.json::<OaiResponse>().await?;

    Ok(response)
}

pub async fn stream_request(payload: OaiPayload) -> Result<OaiMsg, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let uri = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    let client = Client::new();
    let response = client
        .post(uri)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut msg_buf = String::new();
    while let Some(item) = stream.next().await {
        let item = item?; 
        let event = std::str::from_utf8(&item)?;
        for chunk in event.split("\n\n") {
            if let Some(data) = chunk.strip_prefix("data: ") {
                if data == "[DONE]" {
                    break;
                }
                let data = serde_json::from_str::<OaiStreamResponse>(data).unwrap();
                let choice = data.choices.get(0).expect("No choice returned");
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                    stdout().flush().unwrap();
                    msg_buf.push_str(content);
                }
            }
        }
    }
    println!("");
    Ok(OaiMsg::new(Role::Assistant, msg_buf))
}
