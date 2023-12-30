use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use crate::data::game_data::Game;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameList {
    pub games: Vec<Game>,
}

pub fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)

}

