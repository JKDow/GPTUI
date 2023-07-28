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
    name: String,
    directory: PathBuf, 
}

impl OaiChat {
    pub fn new(model: Model, stream: bool, api_key: String, directory: PathBuf) -> Self {
        let creation = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let name = format!("{}_{}.json", model.to_string(), creation);
        Self {
            messages: Vec::new(),
            model,
            stream,
            creation: creation,
            api_key,
            name,
            directory
        }
    }

    fn from_censored(chat: OaiChatCensored, api_key: String, path: PathBuf) -> Self {
        Self {
            messages: chat.messages,
            model: chat.model,
            stream: chat.stream,
            creation: chat.creation,
            api_key,
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            directory: path.parent().unwrap().to_path_buf(),
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
            true => request::stream_request(payload, &self.api_key).await.unwrap(),
            false => {
                let mut response = request::send_request(payload, &self.api_key).await.unwrap();
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
    /// Saves the curren chat object to a json file
    /// 
    /// For cacheing the file will be the name of the model and the current time
    /// 
    /// Example name: gpt3_turbo_2021-08-01_12-00-00.json
    pub fn save_json(&self) -> Result<(),()> {
        // remove file name from self.path and save in directory
        create_dir_all(&self.directory).unwrap();
        // write json to file to path
        let mut path = self.directory.clone();
        path.push(&self.name);
        let mut file = File::create(path).unwrap();
        let json = serde_json::to_string_pretty::<OaiChatCensored>(&self.into()).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }

    /// ## Overview
    /// Loads a chat object from a json file 
    pub fn load_json(path: PathBuf, api_key: String) -> Result<Self,()> {
        let file = File::open(&path).unwrap();
        let chat: OaiChatCensored = serde_json::from_reader(file).unwrap();
        Ok(Self::from_censored(chat, api_key, path))
    }

    /// ## Overview
    /// Prints all the messages currently in the vector 
    pub fn print_messages(&self) {
        for msg in &self.messages {
            println!("{}", msg);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OaiChatCensored {
    messages: Vec<OaiMsg>,
    model: Model,
    stream: bool,
    creation: String,
}

impl From<&OaiChat> for OaiChatCensored {
    fn from(chat: &OaiChat) -> Self {
        Self {
            messages: chat.messages.clone(),
            model: chat.model.clone(),
            stream: chat.stream.clone(),
            creation: chat.creation.clone(),
        }
    }
}