use std::fs;
use std::collections::HashMap;

/**
 * Extract the number from a line (variant with loop and support array, dynamic programming I guess)
 */
fn extract_number(sentence: &str, number_map: &HashMap<&str, char>) -> u8 {
    let mut chars_vector: Vec<char> = sentence.chars().collect();

    let mut digits_vector: Vec<String> = Vec::new();
    
    while chars_vector.len() > 0 {
        if chars_vector.first().unwrap().is_ascii_digit() {
            digits_vector.push(chars_vector.first().unwrap().to_string());
            chars_vector.remove(0);
            continue;
        }
        
        if chars_vector.len() == 1 || chars_vector.len() == 2 {
            chars_vector.remove(0);
            continue
        };
       
        //Retrieving first 5 characters
        let mut sliced_chars: &[char] = &[' '];
        if chars_vector.len() == 3 {sliced_chars = &chars_vector[..3];}
        if chars_vector.len() == 4 {sliced_chars = &chars_vector[..4];}
        if chars_vector.len() >= 5 {sliced_chars = &chars_vector[..5];}

        let sub_string: String = sliced_chars.iter().collect();
        for (key, value) in number_map.iter() {
            if sub_string.starts_with(key) {
                digits_vector.push(value.to_string());
                break;
            }
        }

        chars_vector.remove(0);
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

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut number_map = HashMap::new();
    number_map.insert("one", '1');
    number_map.insert("two", '2');
    number_map.insert("three", '3');
    number_map.insert("four", '4');
    number_map.insert("five", '5');
    number_map.insert("six", '6');
    number_map.insert("seven", '7');
    number_map.insert("eight", '8');
    number_map.insert("nine", '9');

    let mut sum: u32 = 0;
    for elem in arr {
        if elem.is_empty() {continue};
        sum += extract_number(elem, &number_map) as u32;
    }

    println!("Sum: {}", sum);
}
