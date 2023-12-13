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

fn get_touple(line: &str) -> (String, Vec<u8>) {
    let line_split: Vec<&str> = line.split(" ").collect();
    let numbers_string_split: Vec<&str> = line_split[1].split(",").collect();
    let numbers: Vec<u8> = numbers_string_split.iter().map(|x| x.parse::<u8>().unwrap()).collect();
    return (line_split[0].chars().collect(), numbers)
}

fn is_it_an_arrangement(arrangement: &String, number_vec: &Vec<u8>) -> bool {
    let mut found_numbers: Vec<u8> = Vec::new();
    let split_chars: Vec<&str> = arrangement.split(".").collect();
    for group in split_chars {
        if group.contains("#") {
            found_numbers.push(group.len() as u8);
        }
    }

    if found_numbers == *number_vec {
        return true;
    } else {
        return false;
    }
}

fn are_there_question_marks(line_info: &String) -> i32 {
    for index in 0..line_info.len() {
        if line_info.chars().nth(index).unwrap() == '?' {
            return index as i32;
        }
    }
    
    return -1;
}

fn get_arrangements_counter(line_info: &String, numbers: &Vec<u8>) -> u32 {
    let question_mark_index = are_there_question_marks(line_info);
    if question_mark_index == -1 {
        if is_it_an_arrangement(line_info, numbers) {
            return 1;
        } else {
            return 0;
        }
    } else {
        let mut new_line_info = line_info.clone();
        new_line_info.replace_range(question_mark_index as usize..question_mark_index as usize +1, ".");
        let chance0 = get_arrangements_counter(&new_line_info, numbers);
        new_line_info.replace_range(question_mark_index as usize..question_mark_index as usize +1, "#");
        let chance1 = get_arrangements_counter(&new_line_info, numbers);

        return (chance0 + chance1) as u32;
    }
}

fn get_arrangements(line: &str) -> u32 {
    let touple = get_touple(line);
    println!("\n{}", touple.0);
    let arrangement_counter = get_arrangements_counter(&touple.0, &touple.1);

    println!("Counter = {}", arrangement_counter);
    return arrangement_counter;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut sum_of_arrangements = 0;

    for line in arr_str {
        sum_of_arrangements += get_arrangements(line);
    }

    println!("Sum of arrangements: {}", sum_of_arrangements);
}
