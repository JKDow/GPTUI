use std::{io::{stdin, BufRead, Write}, fs::{File, create_dir_all}, path::PathBuf};
use serde::{Deserialize, Serialize};
use super::{request, objects::{OaiMsg, Model, OaiPayload, Role}};

#[derive(Serialize, Deserialize, Debug)]
pub struct OaiChat {
    pub messages: Vec<OaiMsg>,
    pub model: Model,
    pub stream: bool,
    pub creation: String,
    api_key: String,
    path: PathBuf, 
}

impl OaiChat {
    pub fn new(model: Model, stream: bool, api_key: String, directory: PathBuf) -> Self {
        let creation = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let mut path = directory;
        let name = format!("{}_{}.json", model.to_string(), creation);
        path.push(name);
        Self {
            messages: Vec::new(),
            model,
            stream,
            creation: creation,
            api_key,
            path
        }
    }

    /// ## Overview
    /// Sends a message to the open ai api and returns the message response 
    /// 
    /// The message is added to the messages vector
    /// 
    /// If the stream flag is true then the response will print at it is received 
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

    /// ## Overview
    /// Basic loop for having a conversation with the open ai api
    /// 
    /// The loop will continue until the user enters "exit"
    /// 
    /// If the user enters "save" then the chat object will be saved to a json file
    pub async fn basic_loop(&mut self) {
        let mut buf = String::new();
        loop {
            buf.clear();
            self.save_json().unwrap();
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
        }
    }
    
    /// ## Overview
    /// Saves the curren chat object to a json file under ./chats/
    /// 
    /// The file will be the name of the model and the current time
    /// 
    /// Example name: gpt3_turbo_2021-08-01_12-00-00.json
    pub fn save_json(&self) -> Result<(),()> {
        // remove file name from self.path and save in directory
        let mut directory = self.path.clone();
        directory.pop();
        create_dir_all(&directory).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        // write json to file to path
        let mut file = File::create(&self.path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }

    /// ## Overview
    /// Loads a chat object from a json file 
    pub fn load_json(path: PathBuf) -> Result<Self,()> {
        let file = File::open(path).unwrap();
        let chat: Self = serde_json::from_reader(file).unwrap();
        Ok(chat)
    }

    /// ## Overview
    /// Prints all the messages currently in the vector 
    pub fn print_messages(&self) {
        for msg in &self.messages {
            println!("{}", msg);
        }
    }
}