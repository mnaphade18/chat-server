use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MimeType {
    Document,
    Image,
    Video,
    Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub sender_id: String,
    pub group_id: String,
    pub message_type: MimeType,
}


#[derive(Deserialize)]
pub struct NewMessageInput {
    content: String,
    sender_id: String,
    group_id: String,
    mime_type: Option<MimeType>,
}

impl Message {
    pub fn new(input: NewMessageInput) -> Self {
        let message_type = input.mime_type.unwrap_or(MimeType::Text);

        Message {
            id: Uuid::new_v4().to_string(),
            content: input.content,
            sender_id: input.sender_id,
            group_id: input.group_id,
            message_type: message_type,
        }
    }
}
