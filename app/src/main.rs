use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    name: String,
    path: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GameList {
    games: Vec<Game>,
}

fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // read paths.json file
    let json_file = read_json_file("./paths.json").unwrap();
    // println!("{}", json_file);

    // parse json file
    let game_list: GameList = serde_json::from_str(&json_file).expect("JSON was not well-formatted");

    // print game list
    println!("{:?}", game_list);

    // print type of each path
    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display());
    // }
}