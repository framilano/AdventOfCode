use std::fs;

struct NumberPart {
    number: i32,
    row: usize,
    column: usize,
    len: usize,
    is_engine_part: bool
}

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
                number_parts.push(NumberPart { number: number_vec.join("").parse().unwrap(), row: index, column: start, len: end-start, is_engine_part: false })
            } else {
                jndex += 1;
            }
        }
    }

    return number_parts;
}

fn is_symbol_adjacent(row: usize, column: usize, matrix: &mut Vec<Vec<char>>) -> bool {
    //check left
    if column > 0 {
        if !matrix[row][column-1].is_ascii_digit() && matrix[row][column-1] != '.' {return true;}
    }
    //check right
    if column < matrix[row].len() - 1 {
        if !matrix[row][column+1].is_ascii_digit() && matrix[row][column+1] != '.' {return true;}
    }
    //check down
    if row < matrix.len() - 1 {
        if !matrix[row+1][column].is_ascii_digit() && matrix[row+1][column] != '.' {return true;}
    }
    //check up
    if row > 0 {
        if !matrix[row-1][column].is_ascii_digit() && matrix[row-1][column] != '.' {return true;}
    }

    //Checking diagonals
    //check up-left
    if row > 0 && column > 0 {
        if !matrix[row-1][column-1].is_ascii_digit() && matrix[row-1][column-1] != '.' {return true;}
    }
    //check up-right
    if row > 0 && column < matrix[row].len() - 1 {
        if !matrix[row-1][column+1].is_ascii_digit() && matrix[row-1][column+1] != '.' {return true;}
    }
    //check down-left
    if row < matrix.len() - 1 && column > 0 {
        if !matrix[row+1][column-1].is_ascii_digit() && matrix[row+1][column-1] != '.' {return true;}
    }
    //check down-right
    if row < matrix.len() - 1 && column < matrix[row].len() - 1 {
        if !matrix[row+1][column+1].is_ascii_digit() && matrix[row+1][column+1] != '.' {return true;}
    }

    return false;
}

fn update_is_engine_part_status(number_parts: &mut Vec<NumberPart>, matrix: &mut Vec<Vec<char>>) {
    for number_part in number_parts {
        for index in number_part.column..number_part.column+number_part.len {
            if is_symbol_adjacent(number_part.row, index, matrix) {
                number_part.is_engine_part = true;
                //let's check next NumberPart
                break;
            } 
        }
    }
}

fn extract_engine_parts(matrix: &mut Vec<Vec<char>>) -> Vec<i32> {
    let mut numbers_found: Vec<i32> = Vec::new();

    let mut number_parts: Vec<NumberPart> = get_number_parts(matrix);

    update_is_engine_part_status(&mut number_parts, matrix);

    for number_part in number_parts {
        if number_part.is_engine_part == true {
            numbers_found.push(number_part.number);
        }
    }

    return numbers_found;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();
    arr.pop();

    let height = arr.len();
    let width = arr[0].len();

    let mut matrix: Vec<Vec<char>> = vec![vec![' '; width]; height];

    let mut index = 0;
    for line in arr {
        let mut jndex = 0;
        for letter in line.chars() {
            matrix[index][jndex] = letter;
            jndex += 1;
        }
        index += 1;
    }

    let mut sum_of_parts = 0;

    let numbers_found = extract_engine_parts(&mut matrix);
    
    for number in numbers_found {
        sum_of_parts += number;
    }
    
    println!("Sum of parts: {}", sum_of_parts);
}
