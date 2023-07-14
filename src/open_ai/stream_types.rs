use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OaiStreamDelta {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OaiStreamChoice {
    pub index: Option<usize>,
    pub delta: OaiStreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OaiStreamResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<usize>,
    pub model: Option<String>,
    pub choices: Vec<OaiStreamChoice>,
}