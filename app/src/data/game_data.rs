use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub name: String,
    pub path: String,
}
