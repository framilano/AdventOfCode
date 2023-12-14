use std::fs;

fn get_card_points(line: &str) -> u32 {
    let line_split: Vec<&str> = line.split("|").collect();
    let your_numbers_str = line_split[0];
    let winning_numbers_str: &str = line_split[1];

    let your_numbers: Vec<&str> = your_numbers_str.split(" ").collect();
    let winning_numbers: Vec<&str> = winning_numbers_str.split(" ").collect();

    let mut match_counter = 0;
    for number in your_numbers {
        if number.contains("Card") || number.contains(":") || number.is_empty() {continue;}
        for winning_number in &winning_numbers {
            if winning_number.is_empty() {continue;}
            if number.trim() == winning_number.trim() {
                if match_counter == 0 {
                    match_counter = 1;
                }
                else {
                    match_counter *=2 ;
                }
            }
        }
    }

    return match_counter;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();
    //Removing last empty line
    arr.pop();


    let mut sum_of_points = 0;
    for line in arr {
        sum_of_points += get_card_points(line);
    }

    println!("Sum of points: {}", sum_of_points);
}
