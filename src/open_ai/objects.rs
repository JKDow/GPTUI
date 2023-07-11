use std::fmt::Display;
use serde::{Serialize, Deserialize};

/// ## Overview
/// The model to use for the chat
/// ## Models 
/// - **Gpt4:** The original GPT-4 model
/// - **Gpt4_32k:** GPT-4 with a 32k token vocab
/// - **Gpt3Turbo:** GPT-3.5 Turbo 
/// - **Gpt3Turbo16k:** GPT-3.5 Turbo with a 16k token vocab
#[derive(Serialize, Deserialize, Debug)]
pub enum Model {
    Gpt4,
    Gpt4_32k,
    Gpt3Turbo,
    Gpt3Turbo16k
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = match self {
            Model::Gpt4 => "gpt-4",
            Model::Gpt4_32k => "gpt-4-32k",
            Model::Gpt3Turbo => "gpt-3.5-turbo",
            Model::Gpt3Turbo16k => "gpt-3.5-turbo-16k",
        };
        write!(f, "{}", model)
    }
}

/// ## Overview
/// The role of the message
/// ## Roles
/// - **User:** The user's message
/// - **Assistant:** Msg from GPT
/// - **System/Function:** Not implemented
#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    User,
    Assistant,
    System, 
    Function
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
            Role::Function => "function"
        };
        write!(f, "{}", role)
    }
}

/// ## Overview
/// The main input data for the chat
/// ## Fields
/// - **model:** The model to use for the chat
/// - **messages:** The messages to send to the chat
/// - **max_tokens:** The max number of tokens to generate
/// ## Token Math
/// - 1 token is 4 characters 
/// - 4 characters is aproximately 3/4 of a word
#[derive(Serialize)]
pub struct OaiPayload {
    pub model: String,
    pub messages: Vec<OaiMsg>,
    pub max_tokens: u32,
}

impl OaiPayload {
    pub fn new(model: &Model, messages: Vec<OaiMsg>, max_tokens: u32) -> Self {
        Self {
            model: model.to_string(),
            messages,
            max_tokens,
        }
    }

    /// Adds a message to the chain
    pub fn add_message(&mut self, message: OaiMsg) {
        self.messages.push(message);
    }
}

/// ## Overview
/// The message to send to the chat
/// ## Fields
/// - **role:** The role of the message
/// - **content:** The content of the message
/// ## Notes
/// - For the role field, see Role enum
/// - Currently only user and assistant are implemented
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OaiMsg {
    pub role: String,
    pub content: String
}

impl OaiMsg {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role: role.to_string(),
            content
        }
    }
}

/// ## Overview
/// The response from the chat
/// ## Fields
/// - **id:** The id of the chat
/// - **object:** The object of the chat
/// - **created:** The time the chat was created
/// - **choices:** The choices of the chat
/// - **usage:** The usage of the chat
/// ## Notes
/// - For the choices field, see OaiChoices struct
/// - For the usage field, see OaiUsage struct
#[derive(Deserialize, Debug)]
pub struct OaiResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub choices: Vec<OaiChoices>,
    pub usage: OaiUsage,
}

/// ## Overview
/// The choices of the chat
/// ## Fields
/// - **index:** The index of the choice
/// - **message:** The message of the choice
/// - **finish_reason:** The reason the chat finished
/// ## Notes
/// - For the message field, see OaiMsg struct
#[derive(Deserialize, Debug)]
pub struct OaiChoices {
    pub index: u32,
    pub message: OaiMsg,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct OaiUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}