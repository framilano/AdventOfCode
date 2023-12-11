use std::{fs, process::exit};

fn get_input_arr() -> Vec<String> {
    let file = fs::read_to_string("../input");
    if file.is_err() {
        println!("File not found...");
        exit(0);
    }
    let mut arr: Vec<String> = file.ok().unwrap().rsplit("\n").map(|x| String::from(x)).collect();
    arr.reverse();
    //Removing last empty line
    arr.pop();
    
    return arr;
}

fn is_row_empty(line: &str) -> bool {
    for letter in line.chars() {
        if letter != '.' {
            return false;
        }
    }

    return true;
}
fn fill_and_expand(arr_str: Vec<&str>) -> Vec<Vec<char>> {
    let universe_matrix: Vec<Vec<char>> = Vec::new();

    for line in arr_str {
        let chars = line.chars();
        
    }

    return universe_matrix;
}


fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let universe_matrix: Vec<Vec<char>> = fill_and_expand(arr_str);
}
