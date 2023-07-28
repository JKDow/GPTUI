use std::path::PathBuf;

use clap::Parser;

/// Saves a cached chat to a new location
#[derive(Parser, Debug)]
pub struct ChatSaveCmd {
    /// The name of the chat to save
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,
    /// Path to save the chat to 
    #[arg(short, long, value_name = "PATH")]
    pub path: PathBuf,
}