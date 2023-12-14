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

/**
 * Checks if the vector is made of all zeroes
 */
fn are_they_all_zeroes(integer_vec: &Vec<i32>) -> bool {
    for integer in integer_vec {
        if *integer != 0 {return false;}
    }
    return true;
}

/**
 * Given an integer vector return the "subtracted" version of it, i.e 1 2 3 4 -> 1 1 1
 */
fn get_subtracted_vec(integer_vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    if integer_vec.len() == 1 {
        return integer_vec;
    }

    for index in 1..integer_vec.len() {
        new_vec.push(integer_vec[index] - integer_vec[index-1]);
    }

    return new_vec;
}


/**
 * Given a single vector of integers, return the predicted value at the end
 * Recursive, of course
 */
fn get_predicted_value(integer_vec: Vec<i32>) -> i32 {
    if are_they_all_zeroes(&integer_vec) {
        return 0;
    }

    let subtracted_vec = get_subtracted_vec(integer_vec.clone());

    //The second part of this exercise is similar to the first one, just inverted, not (last + predicted_val) but (first - predicted_val)
    return integer_vec.first().unwrap() - get_predicted_value(subtracted_vec)
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut sum_of_values = 0;

    //for each line we get the "extrapolated" value
    for line in arr_str {
        let line_split: Vec<&str> = line.split(" ").collect();
        let integer_vec: Vec<i32> = line_split.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        sum_of_values += get_predicted_value(integer_vec);
    }

    println!("Sum of extrapolated values: {}", sum_of_values);
}
