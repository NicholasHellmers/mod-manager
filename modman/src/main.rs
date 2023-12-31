use crate::data::handle_list::handle_list;
use crate::util::vec_tools::rm_first_two_entries;

mod data;
mod util;

fn main() -> Result<(), std::io::Error>{
    // Get args passed to the program
    let args: Vec<String> = std::env::args().collect();
    let query = &args[1];

    match query.as_str() {
        "list" => handle_list(rm_first_two_entries(args)).expect("Invalid argument for list"),
        _ => println!("Invalid command"),
    }

    Ok(())
}