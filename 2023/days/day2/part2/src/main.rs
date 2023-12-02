use std::fs;

fn get_power_of_cubes(line: &str) -> u32 {
    let game_sets_str = line.split(":").collect::<Vec<&str>>()[1].trim();

    let sets_split = game_sets_str.split(";").collect::<Vec<&str>>();
    
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    for set in sets_split {
        let trimmed_set = set.trim();
        let plays_split = trimmed_set.split(",").collect::<Vec<&str>>();

        for play in plays_split {
            let trimmed_play = play.trim();
            let number_of_cubes:u32 = trimmed_play.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
            let color_of_cubes:&str = trimmed_play.split(" ").collect::<Vec<&str>>()[1];

            if color_of_cubes.trim() == "red" && number_of_cubes > max_red {max_red = number_of_cubes;}
            if color_of_cubes.trim() == "green" && number_of_cubes > max_green {max_green = number_of_cubes;}
            if color_of_cubes.trim() == "blue" && number_of_cubes > max_blue {max_blue = number_of_cubes;}
        }
    }
    return max_red * max_green * max_blue;
}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut sum = 0;

    for line in arr {
        if line.is_empty() {continue;}
        let power_of_cubes = get_power_of_cubes(line);
        sum += power_of_cubes;
    }

    println!("Sum of game ids: {}", sum);
}