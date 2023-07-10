pub mod open_ai; 
pub mod request;

use crate::open_ai::objects::{
    OaiPayload, 
    OaiMsg,
    Model,
    Role
};

pub async fn send_msg() {

    let mut payload = OaiPayload::new(Model::Gpt3Turbo, vec![], 100);
    let msg = OaiMsg::new(Role::User, "Hello, how are you?");
    payload.add_message(msg);

    let response = request::send_request(payload).await.unwrap();

    println!("{:#?}", response);
}
