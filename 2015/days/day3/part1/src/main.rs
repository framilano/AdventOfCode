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

fn main() {
    
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut visited_houses_vec:Vec<Coordinate> = Vec::new();
    visited_houses_vec.push(Coordinate { x:0, y: 0 });

    let mut current_x = 0;
    let mut current_y = 0;

    for direction in file.chars() {
        if direction == '^' {
            current_y += 1;
        } else if direction == 'v' {
            current_y -= 1;
        } else if direction == '>' {
            current_x += 1;
        } else if direction == '<' {
            current_x -= 1;
        }

        let current_coord: Coordinate = Coordinate { x: current_x, y: current_y };

        if visited_houses_vec.contains(&current_coord) {continue;}
        else {
            visited_houses_vec.push(current_coord);
        }
    }

    println!("Visited houses: {}", visited_houses_vec.len());
}
