use std::fs;

fn extract_coordinates_from_line(line: &str) -> Option<[[u32; 2]; 2]> {
    let line_split: Vec<&str> = line.split(" ").collect();

    let mut found_coords: [[u32; 2]; 2] = [[0; 2]; 2];

    let mut found_coords_counter = 0;
    for mut word in line_split {
        //It's not a coordinate
        if !word.contains(",") {continue;}
        else {
            word = word.trim();
            let words_split_arr: Vec<&str> = word.split(",").collect();
            let first = words_split_arr[0];
            let second = words_split_arr[1];

            let first_parse_result = first.parse::<u32>();
            let second_parse_result = second.parse::<u32>();
            if !first_parse_result.is_err() && !second_parse_result.is_err() {
                found_coords[found_coords_counter] = [first_parse_result.unwrap(), second_parse_result.unwrap()];
                found_coords_counter += 1;
                if found_coords_counter == 2 {break;}
            }
        }
    }
    if found_coords_counter == 2 {
        return Option::Some(found_coords);
    } else {
        return None;
    }
}

fn update_lights_matrix(line: &str, lights_matrix: &mut [[u8; 1000]; 1000]) {
    let coords_option = extract_coordinates_from_line(line);

    if coords_option.is_none() {return;}

    let coords = coords_option.unwrap();

    if line.contains("turn on") {
        for index in coords[0][0]..=coords[1][0] {
            for jndex in coords[0][1]..=coords[1][1] {
                lights_matrix[index as usize][jndex as usize] = 1;
            }
        }
    } else if line.contains("turn off") {
        for index in coords[0][0]..=coords[1][0] {
            for jndex in coords[0][1]..=coords[1][1] {
                lights_matrix[index as usize][jndex as usize] = 0;
            }
        }
    } else if line.contains("toggle") {
        for index in coords[0][0]..=coords[1][0] {
            for jndex in coords[0][1]..=coords[1][1] {
                if lights_matrix[index as usize][jndex as usize] == 0 {
                    lights_matrix[index as usize][jndex as usize] = 1
                } else {
                    lights_matrix[index as usize][jndex as usize] = 0;
                }

            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut lights_matrix: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for line in arr {
        update_lights_matrix(line, &mut lights_matrix);
    }

    let mut lights_on_counter = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if lights_matrix[i][j] == 1 {
                lights_on_counter += 1
            }
        }
    }

    println!("There are {} lights on", lights_on_counter);
}
