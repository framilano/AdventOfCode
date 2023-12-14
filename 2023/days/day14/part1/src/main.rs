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

fn get_total_load_on_north(matrix: &mut Vec<Vec<char>>) -> usize {
    tilt_north(matrix);

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

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j])
        }
        println!()
    }
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut matrix: Vec<Vec<char>> =  fill_matrix(&arr_str);

    let total_load_on_north = get_total_load_on_north(&mut matrix);

    //print_matrix(&matrix);

    println!("Total load on north: {}", total_load_on_north);

}
