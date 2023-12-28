use serde::{Serialize, Deserialize};
use crate::data::game_data::Game;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameList {
    pub games: Vec<Game>,
}
