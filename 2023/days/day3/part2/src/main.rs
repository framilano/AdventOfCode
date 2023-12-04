use std::{fs, collections::HashMap};

struct NumberPart {
    number: i32,
    row: usize,
    column: usize,
    len: usize,
    adjacent_asterisks: Vec<[usize; 2]>
}

/**
 * Check if the digits in position [row, column] has any adjacent asterisk
 * Returns a Vector of every adjacent asterisk position
 */
fn get_asterisks_positions(row: usize, column: usize, matrix: &mut Vec<Vec<char>>) -> Vec<[usize; 2]> {
    let mut asterisks_vector: Vec<[usize; 2]> = Vec::new(); 
    
    //check left
    if column > 0 {
        if matrix[row][column-1] == '*' {asterisks_vector.push([row, column-1])}
    }
    //check right
    if column < matrix[row].len() - 1 {
        if matrix[row][column+1] == '*' {asterisks_vector.push([row, column+1])}
    }
    //check down
    if row < matrix.len() - 1 {
        if matrix[row+1][column] == '*' {asterisks_vector.push([row+1, column])}
    }
    //check up
    if row > 0 {
        if matrix[row-1][column] == '*' {asterisks_vector.push([row-1, column])}
    }

    //Checking diagonals
    //check up-left
    if row > 0 && column > 0 {
        if matrix[row-1][column-1] == '*' {asterisks_vector.push([row-1, column-1])}
    }
    //check up-right
    if row > 0 && column < matrix[row].len() - 1 {
        if matrix[row-1][column+1] == '*' {asterisks_vector.push([row-1, column+1])}
    }
    //check down-left
    if row < matrix.len() - 1 && column > 0 {
        if matrix[row+1][column-1] == '*' {asterisks_vector.push([row+1, column-1])}
    }
    //check down-right
    if row < matrix.len() - 1 && column < matrix[row].len() - 1 {
        if matrix[row+1][column+1] == '*' {asterisks_vector.push([row+1, column+1])}
    }

    return asterisks_vector;
}

/**
 * Fills the adjacent_asterisks field for every NumberPart in number_parts
 */
fn update_asterisks_position(number_parts: &mut Vec<NumberPart>, matrix: &mut Vec<Vec<char>>) {
    for number_part in number_parts {
        for index in number_part.column..number_part.column+number_part.len {
            
            //Adding all asterisks found for every digit into number_part.adjacent_asterisks, without repetition
            let temp_asterisks: Vec<[usize; 2]> = get_asterisks_positions(number_part.row, index, matrix);
            for asterisk_coord in temp_asterisks {
                if !number_part.adjacent_asterisks.contains(&asterisk_coord) {
                    number_part.adjacent_asterisks.push(asterisk_coord);
                }
            }
        }
    }
}

/**
 * Returns a vector of NumberParts containing every single number inside the given matrix
 */
fn get_number_parts(matrix: &mut Vec<Vec<char>>) -> Vec<NumberPart> {
    let mut number_parts: Vec<NumberPart> = Vec::new();
    for index in 0..matrix.len() {
        let mut jndex = 0;
        while jndex < matrix[index].len() {
            if matrix[index][jndex].is_ascii_digit() {
                let mut number_vec:Vec<String> = Vec::new();
                let start = jndex;
                while jndex < matrix[index].len() {
                    if matrix[index][jndex].is_ascii_digit() {
                        number_vec.push(matrix[index][jndex].to_string());
                        jndex+=1;
                    } else {break;}
                }
                let end = jndex;
                number_parts.push(NumberPart { number: number_vec.join("").parse().unwrap(), row: index, column: start, len: end-start, adjacent_asterisks: Vec::new() })
            } else {
                jndex += 1;
            }
        }
    }

    return number_parts;
}

/**
 * Returns a gears hashmap<asterisk_coordinates, adjacent_numbers> given the input matrix
 */
fn extract_gears(matrix: &mut Vec<Vec<char>>) -> HashMap<[usize; 2], Vec<i32>> {
    let mut gears: HashMap<[usize; 2], Vec<i32>> = HashMap::new();

    //A NumberPart saves every number inside the input matrix 
    let mut number_parts: Vec<NumberPart> = get_number_parts(matrix);

    //A NumberPart also saves every asterisk adjacent to the number
    //We retrieve these positions with function
    update_asterisks_position(&mut number_parts, matrix);

    //Let's fill the gears hashmap 
    //For every number part let's add as key every single adjacent asterisk
    //If the key already exists, then we push the number field contained in number_part
    //If the key doesn't exist yet, then we create the Vector for numbers, and the push the number
    for number_part in number_parts {
        for adjacent_asterisk in number_part.adjacent_asterisks {
            gears.entry(adjacent_asterisk).or_insert(Vec::new()).push(number_part.number);
        }
    }

    return gears;
}

/**
 * Creates input matrix given the width, height and input array
 */
fn create_input_matrix(width: usize, height: usize, input_arr: Vec<&str>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![vec![' '; width]; height];

    let mut index = 0;
    for line in input_arr {
        let mut jndex = 0;
        for letter in line.chars() {
            matrix[index][jndex] = letter;
            jndex += 1;
        }
        index += 1;
    }

    return matrix;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();
    //Removing last empty line
    arr.pop();

    let height = arr.len();
    let width = arr[0].len();

    let mut matrix = create_input_matrix(width, height, arr);

    //A gear ratio is the product between two numbers inside a gear
    let mut sum_of_gears_ratios = 0;
    
    //A Gear is a map with key the asterisk position, and a vector of all number values adjacent to the asterisk
    let gears: HashMap<[usize; 2], Vec<i32>> = extract_gears(&mut matrix);
    for (_k, v) in gears {
        //A real gear only has two adjacent values, so we can ignore the others
        if v.len() != 2 {continue;}
        else {sum_of_gears_ratios += v[0] * v[1]};
    }
    
    println!("Sum of gears: {}", sum_of_gears_ratios);
}
