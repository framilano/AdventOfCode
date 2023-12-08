use std::{process::exit, fs, collections::HashMap};


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

fn fill_nodes_hashmap(clean_arr: Vec<String>) -> HashMap<String, [String; 2]> {
    let mut nodes_map: HashMap<String, [String; 2]> = HashMap::new();
    
    for line in clean_arr {
        let line_split: Vec<&str> = line.split(" ").collect();
        nodes_map.insert(line_split[0].to_string(), [line_split[1].to_string(), line_split[2].to_string()]);
    }

    return nodes_map;
} 

fn main() {
    let arr: Vec<String> = get_input_arr();
    let mut arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut clean_arr = arr_str.iter_mut().map(|x| x.replace("(", "").replace(")", "").replace("= ", "").replace(",", "")).collect::<Vec<String>>();

    let instructions = arr_str[0];

    clean_arr.remove(0);
    clean_arr.remove(0);

    let mut nodes_map: HashMap<String, [String; 2]> = fill_nodes_hashmap(clean_arr);

    let mut next = "AAA".to_string();
    let mut steps = 0;
    let mut instruction_index = 0;
    let len_instructions = instructions.len();    
    let instruction_chars: Vec<char> = instructions.chars().into_iter().collect();
    
    loop {
        if instruction_index == len_instructions {instruction_index = 0}

        if next == "ZZZ" {break;}

        if instruction_chars[instruction_index] == 'L' {
            next = nodes_map.entry(next.to_string()).or_default().get(0).unwrap().to_string();
        } else if instruction_chars[instruction_index] == 'R'{
            next = nodes_map.entry(next.to_string()).or_default().get(1).unwrap().to_string();
        }

        steps += 1;
        instruction_index += 1;
    }

    println!("Steps required: {}", steps);

}
