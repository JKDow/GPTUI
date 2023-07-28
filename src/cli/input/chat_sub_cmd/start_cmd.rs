use clap::Parser;

use crate::open_ai::objects::Model;


/// Starts a new chat session with the AI
#[derive(Parser, Debug)]
pub struct ChatStartCmd {
    /// The model to use for the message
    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<Model>, 
}