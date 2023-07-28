use clap::Parser;


/// Clears the cached chats
#[derive(Parser, Debug)]
pub struct ChatClearCmd {
    pub clear: bool,
}