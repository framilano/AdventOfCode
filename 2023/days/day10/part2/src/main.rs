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

fn find_s(pipes_matrix: &Vec<Vec<char>>, current_position: (usize, usize), previous_position: (usize, usize), loop_path: &mut Vec<(usize, usize)>) {
    let current_letter = pipes_matrix[current_position.0][current_position.1];
    if current_letter == 'S' && !loop_path.is_empty() {
        return
    }

    loop_path.push(current_position);

    match current_letter {
        '-' => {
            //if i come from left i go right, else i go left
            if previous_position.1 == current_position.1-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path)
            }
        },
        '|' => {
            //if i come from top i go down, else top
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path)
            }
        },
        'F' => {
            //if i come from right i go down, else right
            if previous_position.1 == current_position.1+1 {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path)
            }
        },
        'J' => {
            //if i come from north i go left, else north
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path)
            }
        },
        '7' => {
            //if i come from south i go left, else south
            if previous_position.0 == current_position.0+1 {
                find_s(pipes_matrix, (current_position.0, current_position.1-1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0+1, current_position.1), current_position, loop_path)
            }
        },
        'L' => {
            //if i come from north i go right, else north
            if previous_position.0 == current_position.0-1 {
                find_s(pipes_matrix, (current_position.0, current_position.1+1), current_position, loop_path)
            } else {
                find_s(pipes_matrix, (current_position.0-1, current_position.1), current_position, loop_path)
            }
        },
        _ => {
            return
        }
    }
}

fn find_loop(pipes_matrix: &Vec<Vec<char>>, start_position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut loop_path: Vec<(usize, usize)> = Vec::new();

    //let's add the start position first
    loop_path.push(start_position);
    
    //a pipe can only go in one direction, so we need to first understand what kind of pipe is S
    let s_type = detect_s_type(&pipes_matrix, start_position);
    
    //let's go south
    if s_type == '|' || s_type == '7' || s_type == 'F' {
        find_s(pipes_matrix, (start_position.0+1, start_position.1), start_position, &mut loop_path);
    }

    //let's go left
    if s_type == '-' || s_type == 'J' {
        find_s(pipes_matrix, (start_position.0, start_position.1-1), start_position, &mut loop_path);
    }

    //let's go right
    if s_type == 'L' {
        find_s(pipes_matrix, (start_position.0, start_position.1+1), start_position, &mut loop_path);
    }

    return loop_path
}

fn rewrite_matrix(pipes_matrix: &mut Vec<Vec<char>>, loop_path: &Vec<(usize, usize)>) {
    for i in 0..pipes_matrix.len() {
        for j in 0..pipes_matrix[i].len() {
            if !loop_path.contains(&(i, j)) {
                pipes_matrix[i][j] = '.'
            } 
        }
    }
}

#[allow(dead_code)]
fn print_matrix(pipes_matrix: &Vec<Vec<char>>) {
    for i in 0..pipes_matrix.len() {
        for j in 0..pipes_matrix[i].len() {
            print!("{}", pipes_matrix[i][j])
        }
        println!()
    }
}

fn is_it_inside_the_polygon(row: usize, column: usize, pipes_matrix: &Vec<Vec<char>>) -> bool {
    //Using raycasting algorithm to check if a point is inside a polygon
    //count walls on the left of the inspected point and check if they're odd or even

    let mut walls_counter = 0;

    for i in 0..column {
        if pipes_matrix[row][i] == '|' || pipes_matrix[row][i] == 'L' || pipes_matrix[row][i] == 'J' {
            walls_counter += 1;
        }
    }

    if walls_counter % 2 != 0 {
        return true;
    }

    return false;
}

fn get_enclosed_tiles_number(pipes_matrix: &Vec<Vec<char>>, loop_path: &Vec<(usize, usize)>) -> u32{
    let mut enclosed_parts: Vec<(usize, usize)> = Vec::new();

    for i in 0..pipes_matrix.len() {
        for j in 0..pipes_matrix[i].len() {
            if !loop_path.contains(&(i, j)) {
                if is_it_inside_the_polygon(i, j, pipes_matrix) {
                    enclosed_parts.push((i, j));
                }
            }
        }
    } 

    return enclosed_parts.len() as u32;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut pipes_matrix: Vec<Vec<char>> = prepare_matrix(arr_str);
    let start_position: (usize, usize) = find_start(&pipes_matrix);

    let loop_path: Vec<(usize, usize)> = find_loop(&pipes_matrix, start_position);

    //Let's rewrite the matrix to contain only . for tiles outside the loop
    rewrite_matrix(&mut pipes_matrix, &loop_path);

    let enclosed_tiles_number: u32 = get_enclosed_tiles_number(&pipes_matrix, &loop_path);

    println!("Enclosed tiles number: {}", enclosed_tiles_number)
}
