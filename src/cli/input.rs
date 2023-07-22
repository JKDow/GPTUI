use std::path::PathBuf;

use clap::{Parser, Subcommand}; 

use crate::open_ai::objects::Model;

#[derive(Parser)]
#[command(name = "GPTUI")]
#[command(version = "1.0")]
#[command(about = "Terminal UI for ChatGPT")]
#[command(long_about = "Uses the open AI API to create chat GPT in a termianl UI")]
pub struct GptUi {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Msg(MsgCmd),
    Reply(ReplyCmd),
    Chat(ChatCmd),
    Open(OpenCmd),
    Config(ConfigCmd),
    History(HistoryCmd),
}

/// Send a message to the AI for a quick response
#[derive(Parser, Debug)]
pub struct MsgCmd {
    /// The message to send to the AI
    #[arg(value_name = "MESSAGE")]
    pub msg: String,
    /// The model to use for the message
    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<Model>, 
}

/// Reply to a quick chat message (Retains context)
#[derive(Parser, Debug)]
pub struct ReplyCmd {
    /// The message to send to the AI
    pub msg: String,
}

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

/// Starts a new chat session with the AI
#[derive(Parser, Debug)]
pub struct ChatStartCmd {
    /// The model to use for the message
    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<Model>, 
}

/// Saves a cached chat to a new location
#[derive(Parser, Debug)]
pub struct ChatSaveCmd {
    /// The name of the chat to save
    #[arg(short, long, value_name = "NAME")]
    pub name: Option<String>,
    /// Path to save the chat to 
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,
}

/// Loads a chat from a chat file
#[derive(Parser, Debug)]
pub struct ChatLoadCmd {
    /// Path of the chat to load
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,
}

/// Clears the cached chats
#[derive(Parser, Debug)]
pub struct ChatClearCmd {
    pub clear: bool,
}

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

/// Display names of all cached chats
#[derive(Parser, Debug)]
pub struct ChatListCmd {
    pub list: bool,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

#[derive(Parser, Debug)]
pub struct OpenCmd {
    /// The file to open
    pub open: bool
}

#[derive(Parser, Debug)]
pub struct ConfigCmd {
    #[command(subcommand)]
    pub config_sub_cmd: Option<ConfigSubCmd>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubCmd {
    Set,
    Show,
}

#[derive(Parser, Debug)]
pub struct HistoryCmd {
    show_history: bool
}