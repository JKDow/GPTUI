use clap::Parser;
use gpt_ui::{cli::input::GptUi, config::Config};

#[tokio::main]
async fn main() {
    // get message from user 
    println!("Welcome to gpt CLI");
    // get config
    let config: Config = gpt_ui::config::Config::new().unwrap();
    // set env variable with api key
    std::env::set_var("OPENAI_API_KEY", config.gptui.api_key);
    /*
    let args = GptUi::parse();
    match args.subcmd {
        gpt_ui::cli::input::SubCommand::Start(start) => {
            println!("Starting with model: {}", start.model);
        },
        gpt_ui::cli::input::SubCommand::Save(save) => {
            println!("Saving to path: {:?}", save.path);
        },
        gpt_ui::cli::input::SubCommand::Load(load) => {
            println!("Loading from path: {:?}", load.path);
        },
    }
    */
}
