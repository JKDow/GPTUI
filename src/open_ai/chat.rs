use std::io::{stdin, BufRead};

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
        let mut payload = OaiPayload::new(&self.model, self.messages.clone(), 1000);
        let msg = OaiMsg::new(Role::User, msg);
        payload.add_message(msg.clone());
        self.messages.push(msg);
        let mut response = request::send_request(payload).await.unwrap();
        let choice = response.choices.pop().unwrap();
        self.messages.push(choice.message);
        Ok(self.messages.last().unwrap())
    }

    pub async fn basic_loop(&mut self) {
        let mut buf = String::new();
        loop {
            println!("Enter Msg:");
            stdin().lock().read_line(&mut buf).unwrap();
            let msg = buf.trim().to_string();
            if msg == "exit" {
                break;
            }
            let response = self.send_msg(msg).await.unwrap();
            println!("Response:");
            println!("{}", response.content);
            buf.clear();
        }
    }
}