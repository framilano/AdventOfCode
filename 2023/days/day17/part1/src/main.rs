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

fn fill_matrix(arr_str: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = Vec::new();

    for line in arr_str {
        let mut line_matrix: Vec<usize> = Vec::new();
        for letter in line.chars() {
            line_matrix.push(letter.to_string().parse().unwrap());
        }

        matrix.push(line_matrix);
    }

    return matrix;
}

fn get_vertex_list(height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut vertex_list: Vec<(usize, usize)> = Vec::new();

    for i in 0..height {
        for j in 0..width {
            if i == 0 && j == 0 {
                continue;
            }
            vertex_list.push((i as usize, j as usize))
        }
    }

    return vertex_list;
}

fn prepare_distance_map(start: (usize, usize), height: usize, width: usize) -> HashMap<(usize, usize), usize> {
    let mut distance_map: HashMap<(usize, usize), usize> = HashMap::new();
    distance_map.insert(start, 0);

    for i in 0..height {
        for j in 0..width {
            if i == 0 && j == 0 {
                continue;
            } else {
                distance_map.insert((i, j), 999);
            }
        }
    }

    return distance_map;
}

fn get_minimum_candidate(distance_map: &HashMap<(usize, usize), usize>, candidates_list: &mut Vec<(usize, usize)>) -> (usize, usize) {
    let mut minimum_coords: [usize; 2] = [0, 0];
    let mut minimum_value = 999;
    let mut minimum_index = 0;
    for index in 0..candidates_list.len() {
        if distance_map[&candidates_list[index]] < minimum_value {
            minimum_coords = [candidates_list[index].0, candidates_list[index].1];
            minimum_value = distance_map[&candidates_list[index]];
            minimum_index = index;
        }
    }

    candidates_list.remove(minimum_index);

    return (minimum_coords[0], minimum_coords[1])
}

fn get_adjacent_vertices(height: usize, width: usize, current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent_vertices: Vec<(usize, usize)> = Vec::new();
    
    //return top
    if current.0 > 0 {adjacent_vertices.push((current.0-1, current.1))}
    //return down
    if current.0+1 < height {adjacent_vertices.push((current.0+1, current.1))}
    //return left
    if current.1 > 0 {adjacent_vertices.push((current.0, current.1-1))}
    //return right
    if current.1+1 < width {adjacent_vertices.push((current.0, current.1+1))}

    return adjacent_vertices;
}

fn dijkstra_alg(matrix: &Vec<Vec<usize>>, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut distance_map: HashMap<(usize, usize), usize> = prepare_distance_map(start, matrix.len() as usize, matrix[0].len() as usize);
    
    let vertex_list: Vec<(usize, usize)> = get_vertex_list(matrix.len(), matrix[0].len());
    let mut candidates_list: Vec<(usize, usize)> = vertex_list.clone();

    while !candidates_list.is_empty() {
        let current = get_minimum_candidate(&distance_map, &mut candidates_list);
        let adjacent_vertices: Vec<(usize, usize)> = get_adjacent_vertices(matrix.len() as usize, matrix[0].len() as usize, current);
        for adjacent_vertex in adjacent_vertices {
            let partial_sum = distance_map.get(&current).unwrap() + matrix[adjacent_vertex.0][adjacent_vertex.1];
            if partial_sum < *distance_map.get(&adjacent_vertex).unwrap() {
                distance_map.insert(adjacent_vertex, partial_sum);
            }
        }
    }
    


    return distance_map;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let matrix: Vec<Vec<usize>> =  fill_matrix(&arr_str);

    let distance_map = dijkstra_alg(&matrix, (0, 0));

    println!("{:?}", distance_map.get(&(12, 12)));
    
}
