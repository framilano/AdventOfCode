use std::{ fs, process::exit };

struct Map {
    sources: [u64; 2],
    destinations: [u64; 2],
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

fn retrieve_seeds(line: &str) -> Vec<u64>{
    let mut list_of_seeds: Vec<u64> = Vec::new();

    let mut line_split: Vec<&str> = line.split(" ").collect();
    line_split.remove(0);

    for number_str in line_split {
        list_of_seeds.push(number_str.parse::<u64>().ok().unwrap());
    }

    return list_of_seeds;
}

fn parse_seeds_actual_values(seeds_info: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut seeds: Vec<(u64, u64)> = Vec::new();

    let mut index = 0;
    while index < seeds_info.len() {
        seeds.push((seeds_info[index], seeds_info[index] + seeds_info[index+1] - 1));
        index += 2;
    }

    return seeds;
}

fn retrieve_input_map<'a>(arr_str: &Vec<&'a str>, name: &'a str) -> Vec<&'a str> {
    let mut list_of_lines: Vec<&str> = Vec::new();
    
    
    let mut saving: bool = false;
    for line in arr_str {
        if line.contains(name) {
            saving = true;
            continue;
        }

        if saving {
            if line.is_empty() {break;}
            else {
                list_of_lines.push(line);
            }
        }
    }


    return list_of_lines;
}

fn retrieve_vector_of_ranges(input: &Vec<&str>) -> Vec<Map> {

    let mut vector_of_ranges: Vec<Map> = Vec::new();

    for line in input {
        let line_split: Vec<&str> = line.split(" ").collect();
        let destination_start: u64 = line_split[0].parse::<u64>().unwrap();
        let source_start: u64 = line_split[1].parse::<u64>().unwrap();
        let range: u64 = line_split[2].parse::<u64>().unwrap();

        vector_of_ranges.push(Map { 
            destinations: [source_start, source_start + range - 1], 
            sources: [destination_start, destination_start + range - 1] 
        });
    
    }

    return vector_of_ranges;
}

fn get_output_from_range_vector(input: u64, range_vector: &Vec<Map>) -> u64 {
    
    for map in range_vector {
        if input >= map.sources[0] && input <= map.sources[1] {
            let offset = input - map.sources[0];
            return map.destinations[0] + offset;
        } 
    }

    return input;
}

//Checks if a given input is actually a seed given a a vector with tuples of ranges
fn is_it_a_seed(input: u64, seeds: &Vec<(u64, u64)>) -> bool {

    for seed_tuple in seeds {
        if input >= seed_tuple.0 && input <= seed_tuple.1 {
            return true;
        }
    }

    return false;
}

fn main() {
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let seeds_info: Vec<u64> = retrieve_seeds(arr_str[0]);
    let seeds: Vec<(u64, u64)> = parse_seeds_actual_values(&seeds_info);
    
    //Let's first separate the string input in vectors
    let seed_to_soil_input:Vec<&str> = retrieve_input_map(&arr_str, "seed-to-soil");
    let soil_to_fertilizer_input:Vec<&str> = retrieve_input_map(&arr_str, "soil-to-fertilizer");
    let fertilizer_to_water_input:Vec<&str> = retrieve_input_map(&arr_str, "fertilizer-to-water");
    let water_to_light_input:Vec<&str> = retrieve_input_map(&arr_str, "water-to-light");
    let light_to_temperature_input:Vec<&str> = retrieve_input_map(&arr_str, "light-to-temperature");
    let temperature_to_humidity_input:Vec<&str> = retrieve_input_map(&arr_str, "temperature-to-humidity");
    let humidity_to_location_input:Vec<&str> = retrieve_input_map(&arr_str, "humidity-to-location");

    //Same as part1, but we can't try every single seed, it takes way too much time
    //So we read the map in a reverse order trying every location from 0 until we find the first location that gives an existing seed
    //To do this we need to reverse sources and destinations too inside the retrieve_vector_of_ranges method
    let mut matrix_of_maps:Vec<Vec<Map>> = Vec::new();
    matrix_of_maps.push(retrieve_vector_of_ranges(&seed_to_soil_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&soil_to_fertilizer_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&fertilizer_to_water_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&water_to_light_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&light_to_temperature_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&temperature_to_humidity_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&humidity_to_location_input));
    matrix_of_maps.reverse();

    let mut current_location = 0;
    loop {  
        let mut temp_result = current_location;
        for vector_of_map in &matrix_of_maps {
            temp_result = get_output_from_range_vector(temp_result, vector_of_map);
        }
        if is_it_a_seed(temp_result, &seeds) {break;}
        current_location += 1;
    }


    println!("The lowest location number is {:?}", current_location);
}
