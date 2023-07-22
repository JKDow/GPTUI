use clap::Parser;
use gptui::{prelude::*, paths::get_cache_root};
use gptui::cli::input::SubCommand;

#[tokio::main]
async fn main() {
    // get config
    let config: MainConfig = gptui::config::MainConfig::new().unwrap();
    // Read args 
    let args = GptUi::parse();
    
    match args.subcmd {
        SubCommand::Msg(msg_info) => {}
        SubCommand::Reply(reply_info) => {}
        SubCommand::Chat(chat_info) => {}
        SubCommand::Open(open_info) => {}
        SubCommand::Config(config_info) => {}
        SubCommand::History(history_info) => {}
    }
}
