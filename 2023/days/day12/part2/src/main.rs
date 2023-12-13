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

fn get_touple(line: &str) -> (String, Vec<u32>) {
    let line_split: Vec<&str> = line.split(" ").collect();
    let numbers_string_split: Vec<&str> = line_split[1].split(",").collect();
    let numbers: Vec<u32> = numbers_string_split.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    return (line_split[0].chars().collect(), numbers)
}

fn get_multiplied_touple<'a>(touple: &'a (String, Vec<u32>)) -> (String, Vec<u32>) {
    let mut new_descriptor: String = String::new();
    let mut new_number_vec: Vec<u32> = Vec::new();

    for i in 0..5 {
        if i != 4 {
            new_descriptor.push_str(touple.0.as_str());
            new_descriptor.push_str("?");
        } else {
            new_descriptor.push_str(touple.0.as_str());
        }

        new_number_vec.extend(touple.1.clone());
    }

    return (new_descriptor, new_number_vec);
}

fn is_it_an_arrangement(arrangement: &String, number_vec: &Vec<u32>) -> bool {
    let mut found_numbers: Vec<u32> = Vec::new();
    let split_chars: Vec<&str> = arrangement.split(".").collect();
    for group in split_chars {
        if group.contains("#") {
            found_numbers.push(group.len() as u32);
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

fn partial_check(line_info: &String, numbers: &Vec<u32>) -> bool {
    
    return true;
    
}

fn get_arrangements_counter(line_info: &String, numbers: &Vec<u32>) -> u32 {
    let question_mark_index = are_there_question_marks(line_info);
    if question_mark_index == -1 {
        if is_it_an_arrangement(line_info, numbers) {
            return 1;
        } else {
            return 0;
        }
    } else {
        let mut new_line_info = line_info.clone();
        
        let mut chance0 = 0;
        new_line_info.replace_range(question_mark_index as usize..question_mark_index as usize +1, ".");
        if partial_check(&new_line_info, numbers) {
            chance0 = get_arrangements_counter(&new_line_info, numbers);
        }
        
        let mut chance1 = 0;
        new_line_info.replace_range(question_mark_index as usize..question_mark_index as usize +1, "#");
        if partial_check(&new_line_info, numbers) {
            chance1 = get_arrangements_counter(&new_line_info, numbers);
        }

        return (chance0 + chance1) as u32;
    }
}

fn get_arrangements(line: &str) -> u32 {
    println!("{}", line);
    let old_touple: (String, Vec<u32>) = get_touple(line);
    let touple: (String, Vec<u32>) = get_multiplied_touple(&old_touple); 

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