use std::env;
use dotenv::dotenv;
use reqwest::{
    Client, 
    header::{HeaderMap, CONTENT_TYPE}
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Payload {
    model: String,
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct OaiResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<Choices>,
}

#[derive(Deserialize, Debug)]
struct Choices {
    text: String,
    finish_reason: String,
    index: u32,
}


#[tokio::main]
async fn main() {
    let api_key = get_key();
    let url = "https://api.openai.com/v1/completions";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

    let client = Client::new();

    let payload = Payload {
        model: "text-davinci-003".to_string(),
        prompt: "Once upon a time".to_string(),
        max_tokens: 5,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .unwrap()
        .json::<OaiResponse>()
        .await
        .unwrap();
  

    //println!("{:#?}", response.choices[0].text);
    println!("{:#?}", response);
}

fn get_key() -> String{
    dotenv().ok(); 
    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}