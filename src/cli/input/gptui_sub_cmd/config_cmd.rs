use clap::{Parser, Subcommand};


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