use clap::Parser; 
use crate::{open_ai::{objects::Model, chat::OaiChat}, paths::get_cache_root};

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

impl MsgCmd {
    pub async fn run(&self) {
        let mut chat = if self.reply {
            todo!();   
        } else {
            let config = crate::config::MainConfig::new().unwrap();
            let config = config.gptui;
            OaiChat::new_temp(
                self.model.clone().unwrap_or(config.default_model), 
                false, 
                config.api_key, 
                get_cache_root()
            )
        };
        let msg = chat.send_msg(self.msg.clone()).await.unwrap();
    }
}