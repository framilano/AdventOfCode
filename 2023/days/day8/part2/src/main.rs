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

fn get_starting_nodes(nodes_map: &HashMap<String, [String; 2]>) -> Vec<String> {
    let mut nodes_vec: Vec<String> = Vec::new();

    for node in nodes_map {
        if node.0.chars().nth(2).unwrap() == 'A' {
            nodes_vec.push(node.0.to_string());
        }
    }

    return nodes_vec;
}

fn update_mcm_vector(mcm_vector: &mut Vec<u64>, next_vec: &Vec<String>, steps: u64) {
    for i in 0..next_vec.len() {
        if next_vec[i].chars().nth(2).unwrap() == 'Z' {
            if mcm_vector[i] == 0 {
                mcm_vector[i] = steps;
            }
        } 
    }
}

fn is_mcm_vector_complete(mcm_vector: &Vec<u64>) -> bool {
    for elem in mcm_vector {
        if *elem == 0 {return false;}
    }

    return true;
}


fn main() {
    let arr: Vec<String> = get_input_arr();
    let mut arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut clean_arr = arr_str.iter_mut().map(|x| x.replace("(", "").replace(")", "").replace("= ", "").replace(",", "")).collect::<Vec<String>>();

    let instructions = arr_str[0];

    clean_arr.remove(0);
    clean_arr.remove(0);

    let mut nodes_map: HashMap<String, [String; 2]> = fill_nodes_hashmap(clean_arr);

    let mut next_vec = get_starting_nodes(&nodes_map);
    
    let mut instruction_index = 0;
    let len_instructions = instructions.len();    
    let instruction_chars: Vec<char> = instructions.chars().into_iter().collect();
    
    //To find the step where they all end with Z with do the mcm of all their first time ending on Z
    let mut mcm_vector: Vec<u64> = Vec::new();
    //Let's first fill with zeroes the mcm vector
    for _ in 0..next_vec.len() {mcm_vector.push(0)}
    
    
    let mut steps: u64 = 0;
    loop {
        if instruction_index == len_instructions {instruction_index = 0}

        update_mcm_vector(&mut mcm_vector, &next_vec, steps);
        if is_mcm_vector_complete(&mcm_vector) {break;} 
        
        if instruction_chars[instruction_index] == 'L' {
            for index in 0..next_vec.len() {
                next_vec[index] = nodes_map.entry(next_vec[index].to_string()).or_default().get(0).unwrap().to_string();
            }
        } else if instruction_chars[instruction_index] == 'R'{
            for index in 0..next_vec.len() {
                next_vec[index] = nodes_map.entry(next_vec[index].to_string()).or_default().get(1).unwrap().to_string();
            }
        }


        steps += 1;
        instruction_index += 1;
    }


    
    println!("Now the solution is the LCM (Least Common Multiple) between these numbers: {:?}", mcm_vector);
}
