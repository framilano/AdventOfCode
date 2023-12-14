use std::fs;

fn extract_number(sentence: &str) -> u8 {
    let mut digits_vector: Vec<char> = Vec::new();
    let chars_vector: Vec<char> = sentence.chars().collect();
    for letter in chars_vector {
        if letter.is_ascii_digit() {
            digits_vector.push(letter);
        }
    }

    if digits_vector.len() == 0 {return 0;}
    else {
        let string_value = format!("{}{}", digits_vector.first().unwrap(), digits_vector.last().unwrap());
        let combined_number: u8 = string_value.as_str().parse().unwrap();
        return combined_number;
    }
    
}

fn main() {
    let file: String = fs::read_to_string("src/input").expect("Should contain file");

    //Split String with new line as separator, creating a vector
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut sum: u32 = 0;

    for elem in arr {
        if elem.is_empty() {continue};
        sum += extract_number(elem) as u32;
    }

    println!("Sum: {}", sum);
}
