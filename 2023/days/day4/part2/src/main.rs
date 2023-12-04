use std::{fs, collections::HashMap};

struct CardWinning {
    wins: Vec<u32>,
    left: u32
}

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
                match_counter += 1;
            }
        }
    }

    return match_counter;
}

fn get_card_id(line: &str) -> u32 {
    let line_split: Vec<&str> = line.split(" ").collect();
    for word in line_split {
        if word.contains(":") {
            let card_id = word.replace(":", "").parse::<u32>().unwrap();
            return card_id
        }
    }

    return 0;
}

fn are_no_cards_left(cards_describer_map: &mut HashMap<u32, CardWinning>) -> bool {
    for (_k, v) in cards_describer_map.iter() {
        if v.left != 0 {return false;}
    }
    return true;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();
    //Removing last empty line
    arr.pop();

    let mut cards_describer_map: HashMap<u32, CardWinning> = HashMap::new();

    for line in arr.iter() {
        let card_id = get_card_id(line);
        let wins_counter = get_card_points(line);
        
        let mut wins: Vec<u32> = Vec::new();
        for index in card_id+1..=card_id+wins_counter {
            wins.push(index);
        }
        cards_describer_map.insert(card_id, CardWinning { wins: wins, left: 1 });
    }

    let mut cards_counter = arr.len();
    let original_number_of_cards = cards_describer_map.len();
    loop {
        if are_no_cards_left(&mut cards_describer_map) {break;}
        for index in 1..=original_number_of_cards {
            if cards_describer_map.get_mut(&(index as u32)).unwrap().left == 0 {continue;}
            cards_describer_map.get_mut(&(index as u32)).unwrap().left -= 1;
            
            let win_ids = cards_describer_map.get_mut(&(index as u32)).unwrap().wins.clone();

            for card_id in win_ids {
                cards_counter += 1;
                cards_describer_map.get_mut(&(card_id as u32)).unwrap().left += 1;
            }
            
        }
    }

    println!("Total Scratchcards: {}", cards_counter);
}
