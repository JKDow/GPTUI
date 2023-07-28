pub mod open_ai; 
pub mod cli; 
pub mod config; 
pub mod paths; 


pub mod prelude {
    pub use crate::{
        cli::input::{
            gptui_cmd::GptUi, 
            gptui_sub_cmd::all::*, 
            chat_sub_cmd::all::*}, 
        config::MainConfig, 
        open_ai::{
            objects::Model, 
            chat::OaiChat
        }
    };
}