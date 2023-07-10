use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    let key = get_key();
    println!("{key}");
}

fn get_key() -> String{
    dotenv().ok(); 
    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}