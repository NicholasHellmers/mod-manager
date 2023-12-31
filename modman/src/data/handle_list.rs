use crate::data::game_list_data::get_game_list_and_path;

pub fn handle_list(args_list: Vec<String>) -> Result<(), std::io::Error> {

    
    match args_list[0].as_str() {
        "games" => print_game_list(),
        "mods"  => print_mod_list(&args_list[1]).expect("Invalid argument for list"),
        _ => println!("Invalid argument for list"),
        
    }
    
    Ok(())
}

fn print_game_list() {
    let game_list = get_game_list_and_path().unwrap();
    println!("ID, Game, Path");
    for (i, game) in game_list.games.iter().enumerate() {
        println!("{}. {}, {}", i, game.name, game.path);
    }
}

fn print_mod_list(game: &String) -> Result<(), std::io::Error> {
    let game_list = get_game_list_and_path().unwrap();

    // turn game_list into a hashmap where the key is the game name and the value is the path
    let mut game_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for game in game_list.games {
        game_map.insert(game.name, game.path);
    }

    // Get the direcories within the path of the selected game
    let game_path = game_map.get(game).expect("Invalid game name");

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