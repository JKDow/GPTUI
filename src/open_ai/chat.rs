use std::io::{stdin, BufRead};
use super::{request, objects::{OaiMsg, Model, OaiPayload, Role}};


pub struct Chat {
    messages: Vec<OaiMsg>,
    model: Model,
    stream: bool
}

impl Chat {
    pub fn new(model: Model, stream: bool) -> Self {
        Self {
            messages: Vec::new(),
            model,
            stream
        }
    }

    pub async fn send_msg(&mut self, msg: String) -> Result<&OaiMsg,()> {
        let mut payload = OaiPayload::new(&self.model, self.messages.clone(), 1000, self.stream);
        let msg = OaiMsg::new(Role::User, msg);
        payload.add_message(msg.clone());
        self.messages.push(msg);

        let msg = match self.stream {
            true => request::stream_request(payload).await.unwrap(),
            false => {
                let mut response = request::send_request(payload).await.unwrap();
                let choice = response.choices.pop().unwrap();
                choice.message.unwrap()
            }
        };

        self.messages.push(msg.clone());
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
            println!("Response:");
            let response = self.send_msg(msg).await.unwrap().content.clone();
            if !self.stream {
                println!("{}", response);
            }
            buf.clear();
        }
    }
}