use clap::Parser;


/// Display names of all cached chats
#[derive(Parser, Debug)]
pub struct ChatListCmd {
    pub list: bool,
}