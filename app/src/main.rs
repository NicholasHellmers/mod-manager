use std::fs::File;
use std::io::Read;
use crate::data::game_list_data::GameList;

mod data;

fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), std::io::Error>{
    // read paths.json file
    let json_file = read_json_file("./paths.json").unwrap();
    // println!("{}", json_file);

    // parse json file
    let game_list: GameList = serde_json::from_str(&json_file).expect("JSON was not well-formatted");

    // print game list
    println!("{:?}", game_list);

    // Select a game
    println!("Select a game:");
    for (i, game) in game_list.games.iter().enumerate() {
        println!("{}. {}", i, game.name);
    }

    // Read user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().unwrap();

    // Get the direcories within the path of the selected game
    let game_path = &game_list.games[input].path;
    let paths = std::fs::read_dir(game_path).unwrap();

    // Print the only the directories
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            println!("{:}", path.file_name().unwrap().to_str().unwrap());
        }
    }

    Ok(())
}