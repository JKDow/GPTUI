use gpt_cli::open_ai::{chat::Chat, objects::Model};

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt CLI");
    let mut chat = Chat::new(Model::Gpt3Turbo);
    chat.basic_loop().await;
}
