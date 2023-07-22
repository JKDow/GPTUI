pub mod open_ai; 
pub mod cli; 
pub mod config; 
pub mod paths; 


pub mod prelude {
    pub use crate::{
        cli::{
            input::{GptUi, SubCommand}, 
            command_functions::{
                save_chat, 
                clear_chat_logs
            }
        }, 
        config::MainConfig, 
        open_ai::{
            objects::Model, 
            chat::OaiChat
        }
    };
}