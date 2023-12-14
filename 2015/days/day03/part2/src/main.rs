use std::fs;

struct Coordinate {
    x: i32,
    y: i32
}

impl PartialEq for Coordinate {
    fn eq(&self, coord: &Coordinate) -> bool {
        if self.x == coord.x && self.y ==  coord.y {return true;}
        else {return false;}
    }
}

fn update_visited_houses(visited_houses_vec: &mut Vec<Coordinate>, current_x: &mut i32, current_y: &mut i32, direction: char) {
    if direction == '^' {
        *current_y += 1;
    } else if direction == 'v' {
        *current_y -= 1;
    } else if direction == '>' {
        *current_x += 1;
    } else if direction == '<' {
        *current_x -= 1;
    }
    
    let current_coord: Coordinate = Coordinate { x: *current_x, y: *current_y };
    if visited_houses_vec.contains(&current_coord) {return;}
    else {
        visited_houses_vec.push(current_coord);
    }
}

fn main() {
    
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut real_santa_visited_houses_vec:Vec<Coordinate> = Vec::new();
    real_santa_visited_houses_vec.push(Coordinate { x:0, y: 0 });

    let mut robot_santa_visited_houses_vec:Vec<Coordinate> = Vec::new();
    robot_santa_visited_houses_vec.push(Coordinate { x:0, y: 0 });

    let mut real_santa_current_x = 0;
    let mut real_santa_current_y = 0;
    let mut robot_santa_current_x = 0;
    let mut robot_santa_current_y = 0;

    let mut is_real_santa_turn = true;

    for direction in file.chars() {
        if is_real_santa_turn {
            update_visited_houses(&mut real_santa_visited_houses_vec, &mut real_santa_current_x, &mut real_santa_current_y, direction)
        } else {
            update_visited_houses(&mut robot_santa_visited_houses_vec, &mut robot_santa_current_x, &mut robot_santa_current_y, direction)
        }
        is_real_santa_turn = !is_real_santa_turn;
    }

    let mut houses_counter = real_santa_visited_houses_vec.len();

    for coord in robot_santa_visited_houses_vec.iter() {
        if !real_santa_visited_houses_vec.contains(coord) {houses_counter += 1;}
    }

    println!("Visited houses by robot and real santa: {}", houses_counter);
}
