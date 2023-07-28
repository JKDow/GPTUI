use std::path::PathBuf;

use clap::Parser;
use gptui::cli::commands::chat_session::run_chat_session;
use gptui::{prelude::*, paths::get_cache_root};
use gptui::cli::input::SubCommand;

#[tokio::main]
async fn main() {
    // get config
    let config: MainConfig = gptui::config::MainConfig::new().unwrap();
    // Read args 
    let args = GptUi::parse();
    
}
