use std::{process::exit, fs};

fn get_input_arr() -> Vec<String> {
    let file = fs::read_to_string("input");
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

fn main() {
    let arr_str: Vec<&str> = get_input_arr().iter().map(|x| x.as_str()).collect();
}
