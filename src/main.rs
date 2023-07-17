use gpt_tui::open_ai::{chat::Chat, objects::Model};

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt CLI");
    let mut chat = Chat::new(Model::Gpt3Turbo, true);
    //let mut chat = Chat::load_json("gpt-3.5-turbo_2023-07-14_18-41-17.json").unwrap();
    //chat.print_messages();
    chat.basic_loop().await;
}
