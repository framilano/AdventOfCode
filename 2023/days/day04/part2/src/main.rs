
use std::{fs, collections::HashMap};

struct CardWinning {
    wins: Vec<u32>,
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

fn get_scratchcards_per_card(card_id: u32, cards_describer_map: &HashMap<u32, CardWinning>) -> u32{
    if cards_describer_map.get(&(card_id as u32)).unwrap().wins.len() == 0 {
        return 1;   //the card itself
    } else {
        let mut cards_counter = 1; //the card itself
        for win_card_id in cards_describer_map.get(&(card_id as u32)).unwrap().wins.iter() {
            cards_counter += get_scratchcards_per_card(*win_card_id, cards_describer_map);
        }

        return  cards_counter;
    }
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
        cards_describer_map.insert(card_id, CardWinning { wins: wins });
    }

    let mut cards_counter:u32  = 0;
    let original_number_of_cards = cards_describer_map.len();
    
    for card_id in 1..=original_number_of_cards {
        cards_counter += get_scratchcards_per_card(card_id as u32, &cards_describer_map);
    }

    println!("Total Scratchcards: {}", cards_counter);
}