use clap::Parser; 
use crate::open_ai::objects::Model;

/// Send a message to the AI for a quick response
#[derive(Parser, Debug)]
pub struct MsgCmd {
    /// The message to send to the AI
    #[arg(value_name = "MESSAGE")]
    pub msg: String,
    /// The model to use for the message
    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<Model>, 
    /// Continues the message chain rather than starting a new one
    #[arg(short, long)]
    pub reply: bool
}