use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Converrsation {
    pub messege: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation{
        Conversation {
            messages: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,
    pub text: String,
}