use crate::open_ai::objects::{OaiPayload, OaiResponse};
use dotenv::dotenv;
use reqwest::{header::{HeaderMap, CONTENT_TYPE}, Client};
use std::env;

pub async fn send_request(payload: OaiPayload) -> Result<OaiResponse, Box<dyn std::error::Error>> {
    let api_key = get_key();
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

fn get_key() -> String{
    dotenv().ok(); 
    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}