use std::io::stdin;
use gpt_cli::open_ai::{chat::Chat, objects::Model};

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt cli: enter msg: ");
    let mut chat = Chat::new(Model::Gpt3Turbo);
    loop {
        let input = stdin();
        let mut msg = String::new();
        input.read_line(&mut msg).unwrap();
        let msg = msg.trim().to_string();

        // send msg to gpt
        let response = chat.send_msg(msg).await.unwrap();
        println!("{}", response.content);
        println!("Enter Msg:");
    }

}
