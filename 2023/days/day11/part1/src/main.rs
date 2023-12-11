use std::{fs, process::exit};

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

fn is_row_empty(line: &Vec<char>) -> bool {
    for letter in line {
        if *letter != '.' {
            return false;
        }
    }

    return true;
}

fn is_column_empty(universe_matrix: &Vec<Vec<char>>, column: usize) -> bool {
    for row in universe_matrix {
        if row[column] != '.' {
            return false;
        }
    }

    return true;
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

fn expand_rows(universe_matrix: &mut Vec<Vec<char>>) {
    let mut i = 0;
    while i < universe_matrix.len() {
        if is_row_empty(&universe_matrix[i]) {
            let empty_row: Vec<char> = vec!['.'; universe_matrix[0].len()];
            universe_matrix.insert(i, empty_row);
            i+=1;
        }
        i+=1;
    }
}

fn expand_columns(universe_matrix: &mut Vec<Vec<char>>) {
    let mut i = 0;
    while i < universe_matrix[0].len() {
        if is_column_empty(&universe_matrix, i) {
            for line in &mut *universe_matrix {
                line.insert(i, '.');
            }
            i+=1;
        }
        i+=1;
    }
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

fn find_shortest_path(couple: (Node, Node)) -> i32 {
    if couple.0.row == couple.1.row && couple.0.column == couple.1.column {
        return 0;
    }
    
    let row_dist = (couple.1.row - couple.0.row).abs();
    let column_dist = (couple.1.column - couple.0.column).abs();
    if couple.0.row == couple.1.row {
        return column_dist;
    }
    if couple.0.column == couple.1.column {
        return row_dist;
    }

    return 1 + find_shortest_path((Node { row: couple.0.row+1, column: couple.0.column }, couple.1));
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let mut universe_matrix: Vec<Vec<char>> = fill_matrix(arr_str);
    expand_rows(&mut universe_matrix);
    expand_columns(&mut universe_matrix);

    add_numbers(&mut universe_matrix);
    
    let couples: Vec<(Node, Node)> = get_couples(&universe_matrix);
    //println!("{:?}", couples);
    //print_matrix(&universe_matrix);
    

    let mut sum: i32 = 0;
    for couple in couples {
        sum += find_shortest_path(couple);
    }

    println!("Sum: {}", sum);
}
