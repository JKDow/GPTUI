use crate::request;
use super::objects::{OaiMsg, Model, OaiPayload, Role};


pub struct Chat {
    messages: Vec<OaiMsg>,
    model: Model,
}

impl Chat {
    pub fn new(model: Model) -> Self {
        Self {
            messages: Vec::new(),
            model,
        }
    }

    pub async fn send_msg(&mut self, msg: String) -> Result<&OaiMsg,()> {
        let mut payload = OaiPayload::new(&self.model, self.messages.clone(), 100);
        let msg = OaiMsg::new(Role::User, msg);
        payload.add_message(msg.clone());
        self.messages.push(msg);
        let mut response = request::send_request(payload).await.unwrap();
        let choice = response.choices.pop().unwrap();
        self.messages.push(choice.message);
        Ok(self.messages.last().unwrap())
    }
}