use std::{fs, process::exit};

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
            sources: [source_start, source_start + range - 1], 
            destinations: [destination_start, destination_start + range - 1] 
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

fn main() {
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let seeds: Vec<u64> = retrieve_seeds(arr_str[0]);
    
    //Let's first separate the string input in vectors
    let seed_to_soil_input:Vec<&str> = retrieve_input_map(&arr_str, "seed-to-soil");
    let soil_to_fertilizer_input:Vec<&str> = retrieve_input_map(&arr_str, "soil-to-fertilizer");
    let fertilizer_to_water_input:Vec<&str> = retrieve_input_map(&arr_str, "fertilizer-to-water");
    let water_to_light_input:Vec<&str> = retrieve_input_map(&arr_str, "water-to-light");
    let light_to_temperature_input:Vec<&str> = retrieve_input_map(&arr_str, "light-to-temperature");
    let temperature_to_humidity_input:Vec<&str> = retrieve_input_map(&arr_str, "temperature-to-humidity");
    let humidity_to_location_input:Vec<&str> = retrieve_input_map(&arr_str, "humidity-to-location");

    //let's insert inside a Vector all Vectors of {sources, destination} Map
    let mut matrix_of_maps:Vec<Vec<Map>> = Vec::new();
    matrix_of_maps.push(retrieve_vector_of_ranges(&seed_to_soil_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&soil_to_fertilizer_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&fertilizer_to_water_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&water_to_light_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&light_to_temperature_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&temperature_to_humidity_input));
    matrix_of_maps.push(retrieve_vector_of_ranges(&humidity_to_location_input));

    //List of all found locations per seed
    //Per each seed we compute the path toward location
    let mut list_of_locations: Vec<u64> = Vec::new();
    
    let mut counter = 0;
    for seed in seeds {
        list_of_locations.push(seed);
        for vector_of_map in &matrix_of_maps {
           list_of_locations[counter] = get_output_from_range_vector(list_of_locations[counter], vector_of_map);
        }
        counter += 1;
    }

    println!("The lowest location number is {:?}", list_of_locations.iter().min().unwrap());
}
