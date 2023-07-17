use clap::Parser;
use gpt_ui::{cli::input::GptUi, config::Config, open_ai::{objects::Model, chat::Chat}};

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt CLI");
    // get config
    let config: Config = gpt_ui::config::Config::new().unwrap();
    // Read args 
    let args = GptUi::parse();
    match args.subcmd {
        gpt_ui::cli::input::SubCommand::Start(start) => {
            let model = match start.model {
                Some(model) => Model::from_str(&model).unwrap(),
                None => config.gptui.default_model
            };
            let mut chat = Chat::new(model, config.gptui.stream);
            chat.basic_loop().await;
        },
        gpt_ui::cli::input::SubCommand::Save(save) => {
            println!("Saving to path: {:?}", save.path);
        },
        gpt_ui::cli::input::SubCommand::Load(load) => {
            println!("Loading from path: {:?}", load.path);
        },
    }
}
