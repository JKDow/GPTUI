use clap::{Parser, Subcommand}; 
use crate::cli::input::gptui_sub_cmd::all::*;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Msg(MsgCmd),
    Chat(ChatCmd),
    Open(OpenCmd),
    Config(ConfigCmd),
    History(HistoryCmd),
}

#[derive(Parser)]
#[command(name = "GPTUI")]
#[command(version = "1.0")]
#[command(about = "Terminal UI for ChatGPT")]
#[command(long_about = "Uses the open AI API to create chat GPT in a termianl UI")]
pub struct GptUi {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

impl GptUi {
    pub fn run(&self) {
        match &self.subcmd {
            SubCommand::Msg(msg_cmd) => {
                msg_cmd.run();
            },
            SubCommand::Chat(chat_cmd) => {
                chat_cmd.run();
            },
            SubCommand::Open(open_cmd) => {
                open_cmd.run();
            },
            SubCommand::Config(config_cmd) => {
                config_cmd.run();
            },
            SubCommand::History(history_cmd) => {
                history_cmd.run();
            },
        }
    }
}

