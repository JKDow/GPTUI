use clap::{Parser, Subcommand}; 
use crate::cli::input::chat_sub_cmd::all::*;

/// Used to interact with command line chat sessions
#[derive(Parser, Debug)]
pub struct ChatCmd {
    #[command(subcommand)]
    pub chat_sub_cmd: Option<ChatSubCmd>,
}

#[derive(Subcommand, Debug)]
pub enum ChatSubCmd {
    Start(ChatStartCmd),
    Save(ChatSaveCmd),
    Load(ChatLoadCmd),
    Clear(ChatClearCmd),
    Show(ChatShowCmd),
    List(ChatListCmd),
}