use crate::data::handle_list::handle_list;
use crate::util::vec_tools::rm_first_two_entries;

mod data;
mod util;

fn main() -> Result<(), std::io::Error>{
    // Get args passed to the program
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    println!("Searching for {}", query);

    match query.as_str() {
        "list" => handle_list(rm_first_two_entries(args)).expect("Invalid argument for list"),
        _ => println!("Invalid command"),
    }

    // // Read user input
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input: usize = input.trim().parse().unwrap();

    // // Get the direcories within the path of the selected game
    // let game_path = &game_list.games[input].path;
    // let paths = std::fs::read_dir(game_path).unwrap();

    // // Print the only the directories
    // for path in paths {
    //     let path = path.unwrap().path();
    //     if path.is_dir() {
    //         println!("{:}", path.file_name().unwrap().to_str().unwrap());
    //     }
    // }

    Ok(())
}