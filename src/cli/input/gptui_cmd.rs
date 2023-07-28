use clap::{Parser, Subcommand}; 
use crate::cli::input::gptui_sub_cmd::all::*;


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
    Chat(ChatCmd),
    Open(OpenCmd),
    Config(ConfigCmd),
    History(HistoryCmd),
}
