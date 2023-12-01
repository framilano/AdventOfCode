use std::fs;

/**
 * Recursive function with memoization that scans the sentence and save all numbers it founds, in order
 */
fn parse_numbers_with_memo(sentence: &[char], mut memo: Vec<String>) -> Vec<String> {
    if sentence.len() == 0 {return memo}

    //If it starts with a digit we immediately retrieve the digit and save it
    if sentence.first().unwrap().is_ascii_digit() {
        memo.push(sentence.first().unwrap().to_string());
        return parse_numbers_with_memo(&sentence[1..], memo);
    } else {
        
        //if it starts with a letter we have a possible "stringified" number
        if sentence.len() <= 2 {return parse_numbers_with_memo(&sentence[1..], memo);}  //there's no number with <= 2 letters
        
        //if we have atleast 3 chars, we can try to check the presence for one, two and six
        if sentence.len() >= 3 {
            if sentence[0] == 'o' && sentence[1] == 'n' && sentence[2] == 'e' {
                memo.push(String::from("1"));
            }
            else if sentence[0] == 't' && sentence[1] == 'w' && sentence[2] == 'o' {
                memo.push(String::from("2"));
            }
            else if sentence[0] == 's' && sentence[1] == 'i' && sentence[2] == 'x' {
                memo.push(String::from("6"));
            }
        }

        //if we have atleast 4 chars, we can try to check the presence for four, five and nine
        if sentence.len() >= 4 {
            if sentence[0] == 'f' && sentence[1] == 'o' && sentence[2] == 'u' && sentence[3] == 'r' {
                memo.push(String::from("4"));
            }
            else if sentence[0] == 'f' && sentence[1] == 'i' && sentence[2] == 'v' && sentence[3] == 'e' {
                memo.push(String::from("5"));
            }
            else if sentence[0] == 'n' && sentence[1] == 'i' && sentence[2] == 'n' && sentence[3] == 'e' {
                memo.push(String::from("9"));
            }
        }
        
        //if we have atleast 5 chars, we can try to check the presence for 3, 7, 8
        if sentence.len() >= 5 {
            if sentence[0] == 't' && sentence[1] == 'h' && sentence[2] == 'r' && sentence[3] == 'e' && sentence[4] == 'e'{
                memo.push(String::from("3"));
            }
            else if sentence[0] == 's' && sentence[1] == 'e' && sentence[2] == 'v' && sentence[3] == 'e' && sentence[4] == 'n' {
                memo.push(String::from("7"));
            }
            else if sentence[0] == 'e' && sentence[1] == 'i' && sentence[2] == 'g' && sentence[3] == 'h' && sentence[4] == 't' {
                memo.push(String::from("8"));
            }
        }

        //After retrieving 
        return parse_numbers_with_memo(&sentence[1..], memo);
    }
}

/**
 * Extract the number from a line
 */
fn extract_number(sentence: &str) -> u8 {
    let chars_vector: Vec<char> = sentence.chars().collect();

    let digits_vector = parse_numbers_with_memo(chars_vector.as_slice(), Vec::<String>::new());

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

    let mut sum: u32 = 0;
    for elem in arr {
        if elem.is_empty() {continue};
        sum += extract_number(elem) as u32;
    }

    println!("Sum: {}", sum);
}
