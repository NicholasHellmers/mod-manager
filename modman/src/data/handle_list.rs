use crate::data::game_list_data::get_game_list_and_path;

pub fn handle_list(args_list: Vec<String>) -> Result<(), std::io::Error> {

    
    match args_list[0].as_str() {
        "games" => print_game_list(),
        _ => println!("Invalid argument for list"),
        
    }
    
    Ok(())
}

fn print_game_list() {
    let game_list = get_game_list_and_path().unwrap();
    println!("ID, Game Name, Path");
    for (i, game) in game_list.games.iter().enumerate() {
        println!("{}. {}, {}", i, game.name, game.path);
    }
}

