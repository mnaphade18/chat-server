use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}
