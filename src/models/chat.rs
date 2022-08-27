use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: String,
    pub users: Vec<String>,
    pub name: String,
}
