use std::collections::HashMap;
use std::process::exit;
use std::fs;

#[derive(Debug)]
struct Lens {
    label: String,
    lens_focusing_power: u32,
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

fn get_hash(word: &str) -> u32 {
    let mut current_value = 0;

    for letter in word.chars() {
        current_value += letter as u32;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

fn remove_entry(lens_vector: &mut Vec<Lens>, original_word: &str) {
    let mut removed_index = 0;
    let mut found = false;
    for index in 0..lens_vector.len() {
        if lens_vector[index].label == original_word {
            found = true;
            removed_index = index;
            break;
        }
    }
    if found {
        lens_vector.remove(removed_index);
    }
}

fn add_lens(lens_vector: &mut Vec<Lens>, original_word: &str, lens_power: u32) {
    let mut found = false;
    for lens in &mut *lens_vector {
        if lens.label == original_word {
            lens.lens_focusing_power = lens_power;
            found = true;
        }
    }

    if !found {
        lens_vector.push(Lens { label: String::from(original_word), lens_focusing_power: lens_power })
    }
}

fn fill_boxes(word: &str, boxes: &mut HashMap<u32, Vec<Lens>>) {
    if word.contains("-") {
        let original_word = word.split("-").collect::<Vec<&str>>()[0];
        let hash = get_hash(original_word);
        boxes.entry(hash).and_modify(|vect| remove_entry(vect, original_word));
    } else if word.contains("=") {
        let original_word = word.split("=").collect::<Vec<&str>>()[0];
        let lens_power: u32 = word.split("=").collect::<Vec<&str>>()[1].parse().unwrap();
        let hash = get_hash(original_word);
        add_lens(boxes.entry(hash).or_insert(Vec::new()), original_word, lens_power);
    }
}

fn get_total_focusing_power(boxes: &HashMap<u32, Vec<Lens>>) -> u32 {
    let mut sum_of_powers = 0;

    for (k, v) in boxes {
        let mut counter = 1;
        for lens in v {
            sum_of_powers += (k + 1) * lens.lens_focusing_power * counter;
            counter += 1;
        }
    }

    return sum_of_powers;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();
    let mut input_vector: Vec<&str> = Vec::new();
    for line in arr_str {
        input_vector.extend::<Vec<&str>>(line.split(",").collect());
    }

    input_vector.iter_mut().for_each(|x| *x = x.trim());

    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();


    for word in input_vector {
        fill_boxes(word, &mut boxes);
    }

    println!("Sum of focusing power: {}", get_total_focusing_power(&boxes));
}
