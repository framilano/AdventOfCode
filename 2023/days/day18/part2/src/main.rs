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

fn get_matrix(arr_str: &Vec<String>) -> (Vec<Vec<char>>, usize, usize) {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut current_index: i64 = 0;
    let mut max_right = current_index;
    let mut max_left = current_index;
    for line in arr_str {
        let line_split: Vec<&str> = line.split(" ").collect();
        if line_split[2].chars().nth(7).unwrap() == '0' {
            current_index = current_index + i64::from_str_radix(&String::from(line_split[2])[2..7], 16).unwrap();
            if max_right < current_index {max_right = current_index }
        }
        if line_split[2].chars().nth(7).unwrap() == '2' {
            current_index = current_index - i64::from_str_radix(&String::from(line_split[2])[2..7], 16).unwrap();
            if max_left > current_index {max_left = current_index}
        }
        
    }
    
    let final_width = max_left.abs() + max_right + 1;

    current_index = 0;
    let mut max_down = current_index;
    let mut max_up = current_index;
    for line in arr_str {
        let line_split: Vec<&str> = line.split(" ").collect();
        if line_split[2].chars().nth(7).unwrap() == '1' {
            current_index = current_index + i64::from_str_radix(&String::from(line_split[2])[2..7], 16).unwrap();
            if max_down < current_index {max_down = current_index }
        }
        if line_split[2].chars().nth(7).unwrap() == '3' {
            current_index = current_index - i64::from_str_radix(&String::from(line_split[2])[2..7], 16).unwrap();
            if max_up > current_index {max_up = current_index}
        }
        
    }
    
    let final_height = max_up.abs() + max_down + 1;

    for _ in 0..final_height {
        let mut i:Vec<char> = Vec::new();
        for _ in 0..final_width {
            i.push('.');
        }
        matrix.push(i);
    }

    return (matrix, max_up.abs() as usize, max_left.abs() as usize,);
}

fn draw_path(matrix: &mut Vec<Vec<char>>, arr: &Vec<String>, row_start: usize, column_start: usize) {
    let mut current_row = row_start;
    let mut current_col: usize = column_start;
    for line in arr {
        let line_split:Vec<&str> = line.split(" ").collect();

        if line_split[0] == "R" {
            let steps: usize = line_split[1].parse::<usize>().unwrap();
            for i in 0..steps {
                matrix[current_row][current_col + 1 + i] = '#';
            }
            current_col = current_col + steps;
        }
        else if line_split[0] == "L" {
            let steps: usize = line_split[1].parse::<usize>().unwrap();
            for i in 0..steps {
                matrix[current_row][current_col - 1 - i] = '#';
            }
            current_col = current_col - steps;
        }
        else if line_split[0] == "D" {
            let steps: usize = line_split[1].parse::<usize>().unwrap();
            for i in 0..steps {
                matrix[current_row + 1 + i][current_col] = '#';
            }
            current_row = current_row + steps;
        }
        else if line_split[0] == "U" {
            let steps: usize = line_split[1].parse::<usize>().unwrap();
            for i in 0..steps {
                matrix[current_row - 1 - i][current_col] = '#';
            }
            current_row = current_row - steps;
        }
    }
}

fn flood(matrix: &mut Vec<Vec<char>>, row: i32, column: i32) {
    if row < 0 || column < 0 || row > (matrix.len() - 1) as i32 || column > (matrix[0].len() - 1) as i32 {
        return;
    }

    
    if matrix[row as usize][column as usize] == '#' || matrix[row as usize][column as usize] == '0' {
        return
    }

    matrix[row as usize][column as usize] = '0';

    flood(matrix, row-1, column);
    flood(matrix, row+1, column);
    flood(matrix, row, column -1);
    flood(matrix, row, column+1)

}

fn flood_outside(matrix: &mut Vec<Vec<char>>) {
    for i in 0..matrix[0].len() {
        flood(matrix, 0 as i32, i as i32);
    }

    for i in 0..matrix[0].len() {
        flood(matrix, (matrix.len() - 1) as i32, i as i32);
    }

    for i in 0..matrix.len() {
        flood(matrix, i as i32, 0 as i32);
    }

    for i in 0..matrix.len() {
        flood(matrix, i as i32, (matrix[0].len() - 1) as i32);
    }
}

fn count_outsiders(matrix: & Vec<Vec<char>>) -> i32 {
    let mut counter = 0;

    for row in matrix {
        for column in row {
            if *column == '0' {
                counter += 1;
            }
        }
    }

    return counter;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let matrix_info = get_matrix(&arr);
    let mut matrix = matrix_info.0;
    let row_start = matrix_info.1;
    let column_start = matrix_info.2;
    draw_path(&mut matrix, &arr, row_start, column_start);

    //Flooding with 0 whatever is outside the perimeter
    flood_outside(&mut matrix);

    //let's get the whole matrix area and remove the flooded zeroes count
    let area = matrix.len() * matrix[0].len();

    let count_outsiders = count_outsiders(&matrix);

    println!("It can hold {} cubic meters of lava", area-count_outsiders as usize);
}
