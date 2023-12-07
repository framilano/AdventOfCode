use std::{process::exit, fs, collections::HashMap, cmp::Ordering};

struct Hand {
    hand: String, 
    bid: u64,
    hand_type: u8
}

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

fn fill_letter_map(hand: &str) -> HashMap<char, u8> {
    let mut letter_map: HashMap<char, u8> = HashMap::new();

    for letter in hand.chars() {
        *letter_map.entry(letter).or_insert(0) += 1;
    }

    if hand.contains("J") && !(*letter_map.get(&'J').unwrap() == 5 as u8) {
        let number_of_jollys = letter_map.get(&'J').unwrap();
        let max_k = letter_map.clone().into_iter().max_by(|a, b| {
            if a.0 == 'J' {return Ordering::Less}
            if b.0 == 'J' {return Ordering::Greater}
            if a.1 < b.1 {return Ordering::Less}
            else if a.1 > b.1 {return Ordering::Greater}
            else {return Ordering::Equal}
        }).unwrap();
        
        *letter_map.entry(max_k.0).or_insert(0) += *number_of_jollys;
        *letter_map.entry('J').or_insert(0) = 0;
    }

    return letter_map;
}

fn parse_hand_type(hand: &str) -> u8 {
    let letter_map = fill_letter_map(hand);

    //five_of_a_kind and four_of_a_kind
    for (_k, v) in letter_map.iter() {
        if *v == 5 {return 7}
        if *v == 4 {return 6}
    }

    //full_house
    let mut previous_k_value = 0;
    for (_k, v) in letter_map.iter() {
        if previous_k_value == 0 {previous_k_value = *v}
        if (previous_k_value == 3 && *v == 2) || (previous_k_value == 2 && *v == 3){
            return 5
        }
    }
    
    //three_of_a_kind
    for (_k, v) in letter_map.iter() {
        if *v == 3 {return 4}
    }

    //two_pair
    let mut found_first = false;
    for (_k, v) in letter_map.iter() {
        if *v == 2 && !found_first {
            found_first = true;
            continue;
        }
        if *v == 2 && found_first {return 3}
    }
    
    //one_pair
    for (_k, v) in letter_map.iter() {
        if *v == 2 {return 2}
    }

    //high_card
    return 1;
}

fn get_numeric_value_letter(letter: char) -> u8 {
    if letter == 'A' {return 13}
    if letter == 'K' {return 12}
    if letter == 'Q' {return 11}
    if letter == 'T' {return 10}
    if letter == 'J' {return 1}

    return letter.to_digit(10).unwrap() as u8;
}

fn hands_comparator_function(a: &Hand, b: &Hand) -> Ordering {
    if a.hand_type > b.hand_type {return Ordering::Greater}
    else if a.hand_type < b.hand_type {return Ordering::Less}
    else {
        for index in 0..5 {
            let letter_a_value = get_numeric_value_letter(a.hand.chars().nth(index).unwrap());
            let letter_b_value = get_numeric_value_letter(b.hand.chars().nth(index).unwrap());
            
            if letter_a_value > letter_b_value {return Ordering::Greater}
            else if letter_a_value < letter_b_value {return Ordering::Less}
            else {
                continue;
            }
        }

        return Ordering::Equal;
    }
}

fn get_ordered_hands(arr_str: &Vec<&str>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in arr_str {
        let line_split: Vec<&str> = line.split(" ").collect();
        let hand = line_split[0];
        let bid = line_split[1].parse::<u64>().unwrap();
        let hand_type: u8 = parse_hand_type(hand);
        hands.push(Hand { hand: hand.to_string(), bid: bid, hand_type: hand_type })
    } 

    hands.sort_by(hands_comparator_function);

    return hands
}

fn main() {
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let hands: Vec<Hand> = get_ordered_hands(&arr_str);

    let mut sum_of_products:u64 = 0;
    let mut index = 0;
    while index < hands.len() {
        sum_of_products += hands[index].bid * ((index+1) as u64);
        index += 1;
    }
    
    println!("Sum of products: {}", sum_of_products);
}
