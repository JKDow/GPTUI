use std::path::PathBuf;

use clap::Parser;


/// Loads a chat from a chat file
#[derive(Parser, Debug)]
pub struct ChatLoadCmd {
    /// Path of the chat to load
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,
}