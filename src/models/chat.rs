use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: String,
    pub users: Vec<String>,
    pub name: String,
}
