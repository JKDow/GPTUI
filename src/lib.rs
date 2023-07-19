pub mod open_ai; 
pub mod cli; 
pub mod config; 


pub mod prelude {
    pub use crate::{
        cli::{
            input::{GptUi, SubCommand}, 
            command_functions::{
                save_chat, 
                clear_chat_logs
            }
        }, 
        config::Config, 
        open_ai::{
            objects::Model, 
            chat::Chat
        }
    };
}