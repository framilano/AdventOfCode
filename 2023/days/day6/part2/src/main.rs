use std::{fs, process::exit, iter::zip};

fn get_input_arr() -> Vec<String> {
    let file = fs::read_to_string("input");
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

fn extract_time_distance(time_str: &str, distance_str: &str) -> Vec<(u64, u64)> {
    let mut time_dist_vect:Vec<(u64, u64)> = Vec::new();

    let mut times: Vec<&str> = time_str.split(" ").collect();
    let mut distances: Vec<&str> = distance_str.split(" ").collect();

    times.retain(|x| !x.is_empty() && !x.contains(":"));
    distances.retain(|x| !x.is_empty() && !x.contains(":"));

    for (time, distance) in zip(times, distances) {
        time_dist_vect.push((time.parse::<u64>().unwrap(), distance.parse::<u64>().unwrap()));
    }

    return time_dist_vect;
}

fn get_number_of_ways(time_dist_tuple: (u64, u64)) -> u64 {
    let mut number_of_ways = 0;

    for index in 1..=(time_dist_tuple.0 - 1) {
        let speed = index;
        let time_left = time_dist_tuple.0 - index;

        if (time_left * speed) > time_dist_tuple.1 {
            number_of_ways += 1;
        }
    }

    return number_of_ways;
}

fn main() {
    let arr = get_input_arr();
    let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

    let time_dist_vec: Vec<(u64, u64)> = extract_time_distance(arr_str[0], arr_str[1]);

    let mut product_of_ways:u64 = 1;

    for time_dist in time_dist_vec {
        product_of_ways *= get_number_of_ways(time_dist);
    }


    println!("Solution: {}", product_of_ways);
}
