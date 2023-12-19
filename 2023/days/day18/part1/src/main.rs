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

fn get_vertices_list(arr: &Vec<String>) -> (Vec<(i64, i64)>, i64) {
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut perimeter = 0;

    let mut vertices_list: Vec<(i64, i64)> = Vec::new();

    for line in arr {
        let line_split: Vec<&str> = line.split(" ").collect();
        if line_split[0].chars().nth(0).unwrap() == 'R' {                                                           
            current_x += line_split[1].parse::<i64>().unwrap();     //R
            perimeter += line_split[1].parse::<i64>().unwrap();
        } else if line_split[0].chars().nth(0).unwrap() == 'L' {
            current_x -=line_split[1].parse::<i64>().unwrap();      //L
            perimeter += line_split[1].parse::<i64>().unwrap();
        } else  if line_split[0].chars().nth(0).unwrap() == 'D' {
            current_y -= line_split[1].parse::<i64>().unwrap();     //D
            perimeter += line_split[1].parse::<i64>().unwrap();
        } else if line_split[0].chars().nth(0).unwrap() == 'U' {
            current_y += line_split[1].parse::<i64>().unwrap();     //U
            perimeter += line_split[1].parse::<i64>().unwrap();
        }

        vertices_list.push((current_x, current_y));
    }

    return (vertices_list, perimeter)
}

fn gauss_area(list_of_vertices: &Vec<(i64, i64)>) -> i64 {
    let mut partial_sum: i64 = 0;

    for n in 1..list_of_vertices.len() {
        partial_sum += list_of_vertices[n-1].0 * list_of_vertices[n].1
    }

    partial_sum += list_of_vertices.last().unwrap().0*list_of_vertices.first().unwrap().1;
    
    for n in 1..list_of_vertices.len() {
        partial_sum -= list_of_vertices[n].0 * list_of_vertices[n-1].1
    }

    partial_sum -= list_of_vertices.last().unwrap().1*list_of_vertices.first().unwrap().0;


    return partial_sum.abs() / 2;
}

fn main() {
    //Input parsing
    let arr: Vec<String> = get_input_arr();
    let (vertices_list, perimeter): (Vec<(i64, i64)>, i64) = get_vertices_list(&arr);

    //Shoelace formula
    let area: i64 = gauss_area(&vertices_list);

    //Shoelace funziona su numeri reali, quindi come se partisse da (0.5, 0.5) per ogni vertice

    //Va inserito il teorema di Pick
    //Area = Punti interni + Perimetro/2 - 1

    //Dato che a noi interessano i punti interni + il perimetro, la formula diventa
    //Punti Interni + Perimetro = Area - Perimetro/2 + 1

    let result = area  + perimeter / 2 + 1;
    
    println!("It can hold {} cubic meters of lava", result);
}
