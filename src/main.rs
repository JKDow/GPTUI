use clap::Parser;
use gptui::prelude::*;

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt CLI");
    // get config
    let config: Config = gptui::config::Config::new().unwrap();
    // Read args 
    let args = GptUi::parse();
    match args.subcmd {
        SubCommand::Start(start) => {
            let model = match start.model {
                Some(model) => Model::from_str(&model).unwrap(),
                None => config.gptui.default_model
            };
            let mut chat = Chat::new(model, config.gptui.stream, config.gptui.api_key, config.gptui.chat_log_directory);
            chat.basic_loop().await;
        },
        SubCommand::Save(save) => {
            println!("Saving last chat to path: {:?}", save.path);
            save_chat(save.path, config.gptui.chat_log_directory);

        },
        SubCommand::Load(load) => {
            println!("Loading from path: {:?}", load.path);
        },
        SubCommand::Clear => {
            println!("This will delete all files in: {}", config.gptui.chat_log_directory.to_str().unwrap());
            println!("Are you sure you want to continue? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                println!("Aborting");
                return;
            } else if input.trim() == "y" {
                println!("Clearing chat logs");
                let n = clear_chat_logs(config.gptui.chat_log_directory);
                println!("Cleared {} chat logs", n);
            } else {
                println!("Invalid input");
                return;
            }
        }
        SubCommand::Continue(continue_cmd) => {
            println!("Continuing from path: {:?}", continue_cmd.path);
            let mut chat = Chat::load_json(continue_cmd.path).unwrap();
            chat.print_messages();
            chat.basic_loop().await;
        }
    }
}
