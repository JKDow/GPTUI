
pub mod msg_cmd;
pub mod chat_cmd;
pub mod open_cmd;
pub mod config_cmd;
pub mod history_cmd;

pub mod all {
    pub use super::msg_cmd::*;
    pub use super::chat_cmd::*;
    pub use super::open_cmd::*;
    pub use super::config_cmd::*;
    pub use super::history_cmd::*;
}