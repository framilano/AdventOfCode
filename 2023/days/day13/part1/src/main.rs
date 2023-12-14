use std::{fs, process::exit};

fn get_input_arr() -> Vec<String> {
    let file = fs::read_to_string("../input");
    if file.is_err() {
        println!("File not found...");
        exit(0);
    }
    let mut arr: Vec<String> = file.ok().unwrap().rsplit("\n").map(|x| String::from(x)).collect();
    arr.reverse();

    return arr;
}

fn get_blocks<'a>(arr_str: &'a Vec<&'a str>) -> Vec<Vec<Vec<char>>> {
    let mut blocks: Vec<Vec<Vec<char>>> = Vec::new();

    let mut block: Vec<Vec<char>> = Vec::new();
    for line in arr_str {
        if line.is_empty() {
            blocks.push(block);
            block = Vec::new();
        } else {
            let mut chars: Vec<char> = Vec::new();
            for char in line.chars() {
                chars.push(char);
            }
            block.push(chars);
        }
    }

    return blocks;
}

fn get_row_differences(block:  &Vec<Vec<char>>, row0: usize, row1: usize) -> u32 {
    let mut differences_counter = 0;
    for index in 0..block[row0].len() {
        if block[row0][index] != block[row1][index] {
            differences_counter += 1;
        }
    }

    return  differences_counter;
}

fn get_column_differences(block:  &Vec<Vec<char>>, column0: usize, column1: usize) -> u32 {
    let mut differences_counter = 0;
    for index in 0..block.len() {
        if block[index][column0] != block[index][column1] {
            differences_counter += 1;
        }
    }

    return  differences_counter;
}

fn get_row_reflections(block:  &Vec<Vec<char>>) -> u32 {
    for index in 1..block.len() {
        let differences = get_row_differences(block, index, index - 1);

        if differences == 0 {
            let mut errors_counter = 0;
            let mut failed = false;
            let distance_from_start = 0 + index;
            let distance_from_end = block.len() - index;
            let mut minimum_dist = distance_from_start;
            if distance_from_start > distance_from_end {
                minimum_dist = distance_from_end;
            }
            for offset in 0..minimum_dist {
                let partial_differences = get_row_differences(block, index+offset, index-1-offset);
                errors_counter += partial_differences;
                if errors_counter > 1 {
                    failed = true;
                    break;
                }
            }
            if !failed && errors_counter == 0 {
                return distance_from_start as u32;
            }
        }
    }

    return 0;
}

fn get_column_reflections(block:  &Vec<Vec<char>>) -> u32 {
    for index in 1..block[0].len() {
        let differences = get_column_differences(block, index, index - 1);

        if differences == 0 {
            let mut errors_counter = 0;
            let mut failed = false;
            let distance_from_start = 0 + index;
            let distance_from_end = block[0].len() - index;
            let mut minimum_dist = distance_from_start;
            if distance_from_start > distance_from_end {
                minimum_dist = distance_from_end;
            }
            for offset in 0..minimum_dist {
                let partial_differences = get_column_differences(block, index+offset, index-1-offset);
                errors_counter += partial_differences;
                if errors_counter > 1 {
                    failed = true;
                    break;
                }
            }
            if !failed && errors_counter == 0 {
                return distance_from_start as u32;
            }
        }
    }

    return 0;
}

fn get_reflections(block: &Vec<Vec<char>>) -> u32 {
    let row_ref: u32 = get_row_reflections(block);
    let col_ref: u32 = get_column_reflections(block);
    if row_ref == 0 {
        return col_ref;
    } else {
        return row_ref * 100;
    }
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();
    
    let mut blocks: Vec<Vec<Vec<char>>> = get_blocks(&arr_str);


    //

    let mut sum_of_notes = 0;
    for block in &mut blocks {
        sum_of_notes += get_reflections(block);
    }
    
    println!("{:?}", sum_of_notes);
}
