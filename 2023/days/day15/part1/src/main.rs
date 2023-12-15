use std::process::exit;
use std::fs;


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

fn get_hash(word: &str) -> u32 {
    let mut current_value = 0;

    for letter in word.chars() {
        current_value += letter as u32;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();
    let mut input_vector: Vec<&str> = Vec::new();
    for line in arr_str {
        input_vector.extend::<Vec<&str>>(line.split(",").collect());
    }

    input_vector.iter_mut().for_each(|x| *x = x.trim());

    let mut sum_of_results = 0;
    for word in input_vector {
        sum_of_results += get_hash(word);
    }

    println!("Sum of results: {}", sum_of_results);
}
