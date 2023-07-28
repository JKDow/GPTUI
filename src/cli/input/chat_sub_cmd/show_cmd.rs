use std::path::PathBuf;

use clap::Parser;


/// Displays a chat to the user. If no chat is specified the most recent cached chat will show. 
#[derive(Parser, Debug)]
pub struct ChatShowCmd {
    /// The name of the cached chat to show
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,
    /// The path of a chat to show
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,
}