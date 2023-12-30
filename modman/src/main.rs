use crate::data::game_list_data::GameList;
use crate::data::game_list_data::read_json_file;

mod data;

fn main() -> Result<(), std::io::Error>{
    // Get args passed to the program
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    // read and parse json file
    let json_file = read_json_file("./paths.json").unwrap();
    let game_list: GameList = serde_json::from_str(&json_file).expect("JSON was not well-formatted");

    // print game list
    println!("{:?}", game_list);

    // Select a game
    println!("Select a game:");
    for (i, game) in game_list.games.iter().enumerate() {
        println!("{}. {}, Path: {}", i, game.name, game.path);
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