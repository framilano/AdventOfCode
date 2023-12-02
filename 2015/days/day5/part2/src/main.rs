use std::fs;

fn double_letters_without_overlapping(line: &str) -> bool {
    let line_vector: Vec<char> = line.chars().collect();
    
    for index in 1..line_vector.len() {
        for jndex in index+2..line_vector.len() {
            if line_vector[jndex] == line_vector[index] && line_vector[jndex-1] == line_vector[index-1] {
                return true;
            }
        }
    }

    return false;
}

fn have_letters_with_different_space(line: &str) -> bool {
    let line_vector: Vec<char> = line.chars().collect();
    for index in 2..line_vector.len() {
        if line_vector[index] == line_vector[index - 2] {return true;}
    }

    return false;
}

fn is_nice(line: &str) -> bool {
    if !double_letters_without_overlapping(line) {
        return false;
    }

    if !have_letters_with_different_space(line) {
        return false;
    }

    return true;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut nice_lines = 0;
    for line in arr {
        if is_nice(line) {nice_lines += 1;}
    }

    println!("Nice lines: {}", nice_lines);
}
