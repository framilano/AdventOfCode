use std::fs;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn parse_game_id(game_id_line: &str) -> i32 {
    return game_id_line.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
}

fn is_game_possible(line: &str) -> i32{
    let game_id_str = line.split(":").collect::<Vec<&str>>()[0].trim();
    let game_sets_str = line.split(":").collect::<Vec<&str>>()[1].trim();
    let game_id = parse_game_id(game_id_str);

    let sets_split = game_sets_str.split(";").collect::<Vec<&str>>();
    for set in sets_split {
        let trimmed_set = set.trim();
        let plays_split = trimmed_set.split(",").collect::<Vec<&str>>();

        for play in plays_split {
            let trimmed_play = play.trim();
            let number_of_cubes:u16 = trimmed_play.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
            let color_of_cubes:&str = trimmed_play.split(" ").collect::<Vec<&str>>()[1];

            if color_of_cubes.trim() == "red" && number_of_cubes > (MAX_RED as u16) {return -1;}
            if color_of_cubes.trim() == "green" && number_of_cubes > (MAX_GREEN as u16) {return -1;}
            if color_of_cubes.trim() == "blue" && number_of_cubes > (MAX_BLUE as u16) {return -1;}
        }
    }
    return game_id;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut sum = 0;

    for line in arr {
        if line.is_empty() {continue;}
        let game_id = is_game_possible(line);
        if game_id != -1 {
            sum += game_id;
        }
    }

    println!("Sum of game ids: {}", sum);
}
