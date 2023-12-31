use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use crate::data::game_data::Game;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameList {
    pub games: Vec<Game>,
}

fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_game_list_and_path() -> Result<GameList, std::io::Error> {
    let json_file = read_json_file("./paths.json").unwrap();
    let game_list: GameList = serde_json::from_str(&json_file).expect("JSON was not well-formatted");
    Ok(game_list)
}