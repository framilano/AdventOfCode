use std::env;
use colored::Colorize;
use rand::{Rng, rngs::ThreadRng};
use std::thread::sleep;
use std::time::Duration;

fn draw_ornament(random_value:f32, position: u8,) {
    if position % 5 == 0 {
        if random_value > 0.0 && random_value < 0.5 {
            print!("{}", "O".bright_red());
        }
        if random_value > 0.5 && random_value < 1.0 {
            print!("{}", "O".bright_white());
        }
        
    } else {
        print!("{}", "*".bright_green());
    }
}

fn draw_tree_row(width: u8, offset: u8, left_offset: u8, rng: &mut ThreadRng, counter: usize) {
    for _ in 0..left_offset + width/2 - offset {
        print!(" ");
    }
    
    let random_value:f32 = rng.gen();

    draw_ornament(random_value, 0+counter as u8);
    for i in 0..offset*2 {
        draw_ornament(random_value, i+1+counter as u8)
    }
    //draw_ornament(random_value, 1);

    println!();
}

fn draw_tree(height: u8, width: u8, left_offset: u8, rng: &mut ThreadRng, counter: usize) {
    for row in 0..height {
        draw_tree_row(width, row, left_offset, rng, counter)
    }
}

fn draw_trunk_row(tree_width: u8, left_offset: u8) {
    let effective_tree_width = tree_width - 1;
    let trunk_width = effective_tree_width/4;
    for _ in 0..left_offset + (tree_width/2 - trunk_width/2) {
        print!(" ");
    }
    
    for _ in 0.. trunk_width {
        print!("{}", "*".yellow());
    }
    
    print!("\n");
    
}

fn draw_trunk(tree_height: u8, tree_width: u8, left_offset: u8) {
    for _ in 0..tree_height/4 {
        draw_trunk_row(tree_width, left_offset);
    }
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    let dims = [7, 15, 21, 35, 42];
    let offsets = [10, 30, 15, 25, 40];
    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(5));
        let tree_height: u8 = dims[counter  % dims.len()];
        let tree_width:u8 = tree_height * 2;  //final width is the final offset tree_height*2 - 1
        let left_offset = offsets[counter % offsets.len()];
        sleep(Duration::from_millis(500));
        print!("\x1B[2J\x1B[1;1H");  
        print!("\n\n\n");
        draw_tree(tree_height, tree_width, left_offset, &mut rng, counter%7);
        draw_trunk(tree_height, tree_width, left_offset);       
        print!("\n\n\n\n");
        counter += 1;
    }

}
