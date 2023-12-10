use std::{fs, process::exit};

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

fn prepare_matrix(arr_str: Vec<&str>) -> Vec<Vec<char>> {
    let mut pipes_matrix: Vec<Vec<char>> = Vec::new();

    for line in arr_str {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        pipes_matrix.push(row);
    }

    return pipes_matrix
}

fn find_start(pipes_matrix: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..pipes_matrix.len() {
        for j in 0..pipes_matrix[i].len() {
            if pipes_matrix[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    return (0, 0)
}

fn detect_s_type(pipes_matrix: &Vec<Vec<char>>, start_position: (usize, usize)) -> char {
    let mut can_i_go_up = true;
    let mut can_i_go_down = true;
    let mut can_i_go_left = true;
    let mut can_i_go_right = true;

    //Checking up
    if start_position.0 == 0 {can_i_go_up = false}
    else if !(pipes_matrix[start_position.0-1][start_position.1] == '|') &&
       !(pipes_matrix[start_position.0-1][start_position.1] == 'F') &&
       !(pipes_matrix[start_position.0-1][start_position.1] == '7') {can_i_go_up = false}

    //Checking down
    if start_position.0 == pipes_matrix.len() - 1 {can_i_go_down = false}
    else if !(pipes_matrix[start_position.0+1][start_position.1] == '|') &&
       !(pipes_matrix[start_position.0+1][start_position.1] == 'L') &&
       !(pipes_matrix[start_position.0+1][start_position.1] == 'J') {can_i_go_down = false}

    //Checking right
    if start_position.1 == pipes_matrix[start_position.0].len() - 1 {can_i_go_right = false} 
    else if !(pipes_matrix[start_position.0][start_position.1+1] == '-') &&
       !(pipes_matrix[start_position.0][start_position.1+1] == '7') &&
       !(pipes_matrix[start_position.0][start_position.1+1] == 'J') {can_i_go_right = false}
    
    //Checking left
    if start_position.1 == 0 {can_i_go_left = false} 
    else if !(pipes_matrix[start_position.0][start_position.1-1] == '-') &&
       !(pipes_matrix[start_position.0][start_position.1-1] == 'L') &&
       !(pipes_matrix[start_position.0][start_position.1-1] == 'F') {can_i_go_left = false}

    if can_i_go_up {
        if can_i_go_down {return '|'}
        if can_i_go_left {return 'J'}
        if can_i_go_right {return 'L'}
        else {return 'X' }
    } else if can_i_go_down {
        if can_i_go_left {return '7'}
        if can_i_go_right {return 'F'}
        else {return 'X' }
    } else if can_i_go_left {
        if can_i_go_right {return '-'}
        else {return 'X' }
    } else {
        return 'X';
    }

}

fn find_s(pipes_matrix: &Vec<Vec<char>>, current_position: (usize, usize), previous_position: (usize, usize), loop_path_counter: &mut u32) {
    let current_letter = pipes_matrix[current_position.0][current_position.1];
    if current_letter == 'S' && !(*loop_path_counter == 0) {
        return
    }

    *loop_path_counter += 1;

    match current_letter {
        '-' => {
            //if i come from left i go right, else i go left
            if previous_position.1 == current_position.1-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path_counter)
            }
        },
        '|' => {
            //if i come from top i go down, else top
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path_counter)
            }
        },
        'F' => {
            //if i come from right i go down, else right
            if previous_position.1 == current_position.1+1 {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path_counter)
            }
        },
        'J' => {
            //if i come from north i go left, else north
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path_counter)
            }
        },
        '7' => {
            //if i come from south i go left, else south
            if previous_position.0 == current_position.0+1 {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path_counter)
            }
        },
        'L' => {
            //if i come from north i go right, else north
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path_counter)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path_counter)
            }
        },
        _ => {
            return
        }
    }
}

fn find_loop(pipes_matrix: &Vec<Vec<char>>, start_position: (usize, usize)) -> u32 {
    let mut loop_path_counter: u32 = 0;
    
    //a pipe can only go in one direction, so we need to first understand what kind of pipe is S
    let s_type = detect_s_type(&pipes_matrix, start_position);
    
    //let's go south
    if s_type == '|' || s_type == '7' || s_type == 'F' {
        find_s(pipes_matrix, (start_position.0+1, start_position.1), start_position, &mut loop_path_counter);
    }

    //let's go left
    if s_type == '-' || s_type == 'J' {
        find_s(pipes_matrix, (start_position.0, start_position.1-1), start_position, &mut loop_path_counter);
    }

    //let's go right
    if s_type == 'L' {
        find_s(pipes_matrix, (start_position.0, start_position.1+1), start_position, &mut loop_path_counter);
    }

    return loop_path_counter;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let pipes_matrix: Vec<Vec<char>> = prepare_matrix(arr_str);
    let start_position: (usize, usize) = find_start(&pipes_matrix);

    let loop_path_counter: u32 = find_loop(&pipes_matrix, start_position);

    if loop_path_counter % 2 == 0 {
        println!("Number of steps from start to farthest point: {}", loop_path_counter /  2)
    } else {
        println!("Number of steps from start to farthest point: {}", loop_path_counter /  2 + 1)
    }

}
