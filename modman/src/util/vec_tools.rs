pub fn rm_first_two_entries(args_list: Vec<String>) -> Vec<String> {
    let mut new_args_list = args_list.clone();
    new_args_list.remove(0);
    new_args_list.remove(0);
    new_args_list
}