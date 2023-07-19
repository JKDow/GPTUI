use clap::Parser; 
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "GPT TUI")]
#[command(version = "1.0")]
#[command(about = "Terminal UI for ChatGPT")]
#[command(long_about = "Uses the open AI API to create chat GPT in a termianl UI")]
pub struct GptUi {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Start(StartCmd),
    Save(SaveCmd),
    Load(LoadCmd),
    Clear,
    Continue(ContinueCmd)
}

#[derive(Parser, Debug)]
pub struct StartCmd {
    pub model: Option<String>
}

#[derive(Parser, Debug)]
pub struct SaveCmd {
    pub path: PathBuf
}

#[derive(Parser, Debug)]
pub struct LoadCmd {
    pub path: PathBuf
}

#[derive(Parser, Debug)]
pub struct ContinueCmd {
    pub path: PathBuf
}

