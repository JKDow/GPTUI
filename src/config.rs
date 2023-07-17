use serde::{Deserialize, Serialize};
use crate::open_ai::objects::Model;

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Config {
    pub gptui: GptUiConfig
}

impl Config {
    /// Reads config file at path and returns Config struct
    /// 
    /// Also sets OPENAI_API_KEY env variable
    pub fn new() -> Result<Self, ()> {
        // read config file at path
        //let path = std::path::Path::new("~/.gptui/config.toml");
        let path = std::path::Path::new("config.toml");
        let config = match std::fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).expect("Failed to parse config file"),
            Err(_) => Config::create_config(path).unwrap()
        };
        // set env variable 
        std::env::set_var("OPENAI_API_KEY", &config.gptui.api_key);
        Ok(config)
    }

    fn create_config(path: &std::path::Path) -> Result<Self, ()> {
        // Create config file at path with default values
        let config = Config::default();
        let content = toml::to_string(&config).expect("Failed to serialize config");
        // Write to file 
        match std::fs::write(path, content) {
            Ok(_) => Ok(config),
            Err(_) => Err(())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GptUiConfig {
    pub api_key: String,
    pub default_model: Model,
    pub stream: bool
}

impl Default for GptUiConfig {
    fn default() -> Self {
        GptUiConfig {
            api_key: String::from(""),
            default_model: Model::Gpt3Turbo,
            stream: true
        }
    }
}
