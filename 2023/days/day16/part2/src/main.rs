use std::collections::HashMap;
use std::process::exit;
use std::fs;

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

fn fill_matrix(arr_str: &Vec<&str>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in arr_str {
        let mut line_matrix: Vec<char> = Vec::new();
        for letter in line.chars() {
            line_matrix.push(letter);
        }

        matrix.push(line_matrix);
    }

    return matrix;
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] != '#' {
                print!(".")
            } else {
                print!("#")
            }
        }
        println!()
    }
}

fn get_energized_total(matrix: &Vec<Vec<char>>, coming_from: char, current_row: i32, current_column: i32, cached_tiles: &mut HashMap<(i32, i32), Vec<char>>) -> i32 {
    
    //Checking out of bounds
    if (current_column == matrix[0].len() as i32) || (current_column == -1) ||
       (current_row == matrix.len() as i32) || (current_row == -1) {
        return 0;
    }

    //Checking tile already visited from that direction
    let mut tile_value = 0;
    if cached_tiles.contains_key(&(current_row, current_column)) {
        if cached_tiles.get(&(current_row, current_column)).unwrap().contains(&coming_from) {
            return 0;
        } else {
            cached_tiles.entry((current_row, current_column)).and_modify(|vec| vec.push(coming_from));
        }
    } else {
        cached_tiles.insert((current_row, current_column), Vec::new() );
        tile_value = 1
    }

    let current_symbol = matrix[current_row as usize][current_column as usize];
    match current_symbol {
        '.' => {
            match coming_from {
                'l' => {return tile_value + get_energized_total(matrix, coming_from, current_row, current_column + 1, cached_tiles)}
                'r' => {return tile_value + get_energized_total(matrix, coming_from, current_row, current_column - 1, cached_tiles)}
                'd' => {return tile_value + get_energized_total(matrix, coming_from, current_row - 1, current_column, cached_tiles)}
                't' => {return tile_value + get_energized_total(matrix, coming_from, current_row + 1, current_column, cached_tiles)},
                _ => {return 0}
            }
        },
        '/' => {
            match coming_from {
                'l' => {return tile_value + get_energized_total(matrix, 'd', current_row - 1, current_column, cached_tiles)}
                'r' => {return tile_value + get_energized_total(matrix, 't', current_row + 1, current_column, cached_tiles)}
                'd' => {return tile_value + get_energized_total(matrix, 'l', current_row, current_column + 1, cached_tiles)}
                't' => {return tile_value + get_energized_total(matrix, 'r', current_row, current_column - 1, cached_tiles)},
                _ => {return 0}

            }
        },
        '\\' => {
            match coming_from {
                'l' => {return tile_value + get_energized_total(matrix, 't', current_row + 1, current_column, cached_tiles)}
                'r' => {return tile_value + get_energized_total(matrix, 'd', current_row - 1, current_column, cached_tiles)}
                'd' => {return tile_value + get_energized_total(matrix, 'r', current_row, current_column - 1, cached_tiles)}
                't' => {return tile_value + get_energized_total(matrix, 'l', current_row, current_column + 1, cached_tiles)},
                _ => {return 0}

            }
        },
        '-' => {
            match coming_from {
                'l' => {return tile_value + get_energized_total(matrix, coming_from, current_row, current_column + 1, cached_tiles)}
                'r' => {return tile_value + get_energized_total(matrix, coming_from, current_row, current_column - 1, cached_tiles)}
                'd' | 't' => {
                    return tile_value + 
                    get_energized_total(matrix, 'l', current_row, current_column + 1, cached_tiles) + 
                    get_energized_total(matrix, 'r', current_row, current_column - 1, cached_tiles)
                },
                _ => {return 0}

            }
        },
        '|' => {
            match coming_from {
                'l' | 'r' => {
                    return tile_value + 
                    get_energized_total(matrix, 'd', current_row - 1, current_column, cached_tiles) + 
                    get_energized_total(matrix, 't', current_row + 1, current_column, cached_tiles)
                }
                'd' => {return tile_value + get_energized_total(matrix, coming_from, current_row - 1, current_column, cached_tiles)}
                't' => {return tile_value + get_energized_total(matrix, coming_from, current_row + 1, current_column, cached_tiles)},
                _ => {return 0}

            }
        },
        _ => {return 0}
    }


}


fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let matrix: Vec<Vec<char>> =  fill_matrix(&arr_str);

    let mut cached_tiles: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    let mut max = 0;
    
    //First row
    for column in 0..matrix[0].len() {
        cached_tiles.clear();
        let current_total = get_energized_total(&matrix, 't', 0, column as i32, &mut cached_tiles);
        if max < current_total {max = current_total}
    }

    //Last row
    for column in 0..matrix[0].len() {
        cached_tiles.clear();
        let current_total = get_energized_total(&matrix, 'd', (matrix.len()-1) as i32, column as i32, &mut cached_tiles);
        if max < current_total {max = current_total}
    }

    //First column
    for row in 0..matrix.len() {
        cached_tiles.clear();
        let current_total = get_energized_total(&matrix, 'l', row as i32, 0, &mut cached_tiles);
        if max < current_total {max = current_total}
    }

    //Last column
    for row in 0..matrix.len() {
        cached_tiles.clear();
        let current_total = get_energized_total(&matrix, 'r', row as i32, (matrix.len() - 1) as i32, &mut cached_tiles);
        if max < current_total {max = current_total}
    }

    println!("Maximized energized counter: {}", max);
}
