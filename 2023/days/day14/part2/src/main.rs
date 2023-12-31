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

fn tilt_north(matrix: &mut Vec<Vec<char>>) {
    for row_index in 0..matrix.len() {
        if row_index == 0 {
            continue;
        }
        for column_index in 0..matrix[row_index].len() {
            if matrix[row_index][column_index] == 'O'  {
                let mut offset = 0;
                while (row_index as i32 - 1 - offset as i32) >= 0 && matrix[row_index-1-offset][column_index] == '.' {
                    matrix[row_index-1-offset][column_index] = 'O';
                    matrix[row_index-offset][column_index] = '.';
                    offset += 1;
                }
            }
        }
    }
}

fn tilt_west(matrix: &mut Vec<Vec<char>>) {
    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            if column_index == 0 {continue;}
            if matrix[row_index][column_index] == 'O'  {
                let mut offset = 0;
                while (column_index as i32 - 1 - offset as i32) >= 0 && matrix[row_index][column_index-1-offset] == '.' {
                    matrix[row_index][column_index-1-offset] = 'O';
                    matrix[row_index][column_index-offset] = '.';
                    offset += 1;
                }
            }
        }
    }
}

fn tilt_south(matrix: &mut Vec<Vec<char>>) {
    for row_index in (0..matrix.len()).rev() {
        if row_index == matrix.len() - 1 {
            continue;
        }
        for column_index in 0..matrix[row_index].len() {
            if matrix[row_index][column_index] == 'O'  {
                let mut offset = 0;
                while (row_index as i32 + 1 + offset as i32) <= (matrix.len() - 1) as i32 && matrix[row_index+1+offset][column_index] == '.' {
                    matrix[row_index+1+offset][column_index] = 'O';
                    matrix[row_index+offset][column_index] = '.';
                    offset += 1;
                }
            }
        }
    }
}

fn tilt_east(matrix: &mut Vec<Vec<char>>) {
    for row_index in (0..matrix.len()).rev() {
        for column_index in (0..matrix[row_index].len()).rev() {
            if column_index == matrix[row_index].len() - 1 {continue;}
            if matrix[row_index][column_index] == 'O'  {
                let mut offset = 0;
                while (column_index as i32 + 1 + offset as i32) <= (matrix[row_index].len() - 1) as i32 && matrix[row_index][column_index+1+offset] == '.' {
                    matrix[row_index][column_index+1+offset] = 'O';
                    matrix[row_index][column_index+offset] = '.';
                    offset += 1;
                }
            }
        }
    }
}

fn get_total_load(matrix: &mut Vec<Vec<char>>) -> usize {
    let mut total_load = 0;
    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            if matrix[row_index][column_index] == 'O' {
                total_load += matrix[row_index].len() - row_index;
            }
        }
    }

    return total_load;
}

fn detect_loop(load_cache: &Vec<usize>) -> Option<(i32, i32)> {
    for index in 0..load_cache.len() {
        for jndex in index+1..load_cache.len() {
            if load_cache[index] == load_cache[jndex] {
                let mut offset = 1;
                let mut failed = false;
                while index + offset < jndex &&  jndex + offset < load_cache.len() {
                    if load_cache[index + offset] == load_cache[jndex + offset] {
                        offset += 1;
                        continue;
                    } else {
                        failed = true;
                        break;
                    }
                }

                if !failed && offset > 2 {
                    return Some((index as i32, jndex as i32));
                }
            }
        }
    }

    return None;
}

fn get_total_load_on_north(matrix: &mut Vec<Vec<char>>) -> usize {
    let mut load_cache: Vec<usize> = Vec::new();
    
    loop {
        tilt_north(matrix);
        tilt_west(matrix);
        tilt_south(matrix);
        tilt_east(matrix);

        load_cache.push(get_total_load(matrix));
        
        //return loop indexes (included, not included)
        let loop_indexes: Option<(i32, i32)> = detect_loop(&load_cache);
        if loop_indexes.is_some() {
            //Loop found, we now have its indexes and its length

            let loop_length = loop_indexes.unwrap().1 - loop_indexes.unwrap().0;

            let distance_from_solution = 999999999 - (((999999999 / loop_length) * loop_length) + loop_indexes.unwrap().0);

            let loop_solution_index = distance_from_solution % loop_length;

            let mut positive_index = loop_indexes.unwrap().0 + loop_solution_index;
            if loop_solution_index < 0 {
                positive_index = loop_indexes.unwrap().0 + loop_solution_index + loop_length;
            }
            
            return load_cache[positive_index as usize];
        }
    }
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j])
        }
        println!()
    }
    println!("\n\n");
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut matrix: Vec<Vec<char>> =  fill_matrix(&arr_str);

    let total_load_on_north = get_total_load_on_north(&mut matrix);

    //print_matrix(&matrix);

    println!("Total load on north after 1000000000 cycles: {}", total_load_on_north);

}
