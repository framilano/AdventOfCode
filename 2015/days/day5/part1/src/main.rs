use std::fs;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn have_three_vowels(line: &str) -> bool {
    let mut vowels_counter = 0;
    for char in line.chars() {
        if VOWELS.contains(&char) {
            vowels_counter += 1;
        }
    }

    if vowels_counter >= 3 {return true;}
    else {return false};
}

fn have_consecutive_chars(line: &str) -> bool {
    let line_vector: Vec<char> = line.chars().collect();
    for index in 1..line_vector.len() {
        if line_vector[index] == line_vector[index - 1] {return true;}
    }

    return false;
}

fn contains_special_sequences(line: &str) -> bool {
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return true;
    }

    return false;
}

fn is_nice(line: &str) -> bool {
    if !have_three_vowels(line) {
        return false;
    }

    if !have_consecutive_chars(line) {
        return false;
    }

    if contains_special_sequences(line) {
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
