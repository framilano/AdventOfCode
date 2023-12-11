use std::{fs, process::exit};

//Exercise comments are in part2

#[derive(Copy, Clone, Debug)]
struct Node {
    row: i32,
    column: i32
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

fn fill_matrix(arr_str: Vec<&str>) -> Vec<Vec<char>> {
    let mut universe_matrix: Vec<Vec<char>> = Vec::new();

    for line in arr_str {
        let chars = line.chars();
        let mut row_chars: Vec<char> = Vec::new();
        for char in chars {
            row_chars.push(char);
        }
        universe_matrix.push(row_chars);
    }

    return universe_matrix;
}

#[allow(dead_code)]
fn print_matrix(universe_matrix: &Vec<Vec<char>>) {
    for i in 0..universe_matrix.len() {
        for j in 0..universe_matrix[0].len() {
            print!("{}", universe_matrix[i][j])
        }
        println!();
    }
}

fn add_numbers(universe_matrix: &mut Vec<Vec<char>>) {
    let mut counter = 1;

    for i in 0..universe_matrix.len() {
        for j in 0..universe_matrix[0].len() {
            if universe_matrix[i][j] == '#' {
                universe_matrix[i][j] = counter.to_string().chars().nth(0).unwrap();
                counter+=1
            }
        }
    }
}

fn get_couples(universe_matrix: &Vec<Vec<char>>) -> Vec<(Node, Node)> {
    let mut couples: Vec<(Node, Node)> = Vec::new();

    let mut nodes: Vec<Node> = Vec::new();

    for index in 0..universe_matrix.len() {
        for jndex in 0..universe_matrix[0].len() {
            if universe_matrix[index][jndex] != '.' {
                nodes.push(Node { row: index as i32, column: jndex as i32 })
            }
        }
    }

    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            couples.push((nodes[i], nodes[j]));
        }
    }

    return couples;
}

fn find_shortest_path(couple: (Node, Node), empty_rows: &Vec<i32>, empty_columns: &Vec<i32>) -> u64 {
    let mut row_diff: u64 = (couple.1.row - couple.0.row).abs() as u64;
    let mut column_diff: u64 = (couple.1.column - couple.0.column).abs() as u64;

    //Adding how many empty rows there are
    for row in empty_rows {
        if *row > couple.0.row && *row < couple.1.row {
            row_diff+=1;
        }
    }
    
    let mut left_column = couple.0.column;
    let mut right_column = couple.1.column;
    if left_column > right_column {
        left_column = couple.1.column;
        right_column = couple.0.column;

    }

    for column in empty_columns {
        if *column > left_column && *column < right_column {
            column_diff+=1;
        }
    }


    return row_diff + column_diff;
}

fn get_empty_rows(universe_matrix: &Vec<Vec<char>>) -> Vec<i32> {
    let mut empty_rows: Vec<i32> = Vec::new();
    for index in 0..universe_matrix.len() {
        let mut is_empty = true;
        for jndex in 0..universe_matrix[index].len() {
            if universe_matrix[index][jndex] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_rows.push(index as i32);
        }
    }
    return empty_rows;
}

fn get_empty_columns(universe_matrix: &Vec<Vec<char>>) -> Vec<i32> {
    let mut empty_columns: Vec<i32> = Vec::new();
    for jndex in 0..universe_matrix[0].len() {
        let mut is_empty = true;
        for index in 0..universe_matrix.len() {
            if universe_matrix[index][jndex] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_columns.push(jndex as i32);
        }
    }
    return empty_columns;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut universe_matrix: Vec<Vec<char>> = fill_matrix(arr_str);

    add_numbers(&mut universe_matrix);
    
    let couples: Vec<(Node, Node)> = get_couples(&universe_matrix);
    //println!("{:?}", couples);
    //print_matrix(&universe_matrix);
    
    let empty_rows = get_empty_rows(&universe_matrix);
    let empty_columns = get_empty_columns(&universe_matrix);


    let mut sum: u64 = 0;
    for couple in couples {
        sum += find_shortest_path(couple, &empty_rows, &empty_columns);
    }

    println!("Sum of paths: {}", sum);
}
