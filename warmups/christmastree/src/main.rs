use std::env;
use colored::Colorize;
use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;

static mut N: i32 = 0;

fn draw_ornament(row_position: u8, column_position: u8) {
    if row_position % 3 == 0 && column_position % 4 == 0{
        if unsafe { N } % 4 == 0 {
            print!("{}", "o".bright_red());
        } else if unsafe { N } % 4 == 1 {
            print!("{}", "o".bright_blue()); 
        } else if unsafe { N } % 4 == 2 {
            print!("{}", "o".bright_purple()); 
        } else if unsafe { N } % 4 == 3 {
            print!("{}", "o".bright_cyan()); 
        }
    } else if (row_position % 3 == 1 || row_position % 3 == 2) && column_position % 4 == (unsafe { N as u8 }%4 ) {
        print!("{}", "*".bright_white());
    } else {
        print!("{}", "*".bright_green());
    }
}

fn draw_tree_row(width: u8, offset: u8, left_offset: u8, row_position: u8) {
    for _ in 0..left_offset + width/2 - offset {
        print!(" ");
    }
    

    draw_ornament(row_position, 0);
    for i in 0..offset*2 {
        draw_ornament(row_position, i + 1);
    }

    println!();
}

fn draw_tree(height: u8, width: u8, left_offset: u8) {
    let mut offset = 0;
    for row in 0..height {
        if row % 5 == 0 && row != 0 {
            offset -= 4;
        } else {
            offset += 2
        }
        draw_tree_row(width, offset, left_offset, row);
    }
}

fn draw_trunk_row(tree_width: u8, left_offset: u8, offset: u8) {
    for _ in 0..left_offset + (tree_width/2 - offset) {
        print!(" ");
    }
    
    for _ in 0.. offset*2 {
        print!("{}", "*".yellow());
    }
    
    print!("\n");
    
}

fn draw_trunk(tree_height: u8, tree_width: u8, left_offset: u8) {
    for offset in 3..=tree_height/4+2 {
        draw_trunk_row(tree_width, left_offset, offset);
    }
}

fn draw_star(tree_width: u8, left_offset: u8) {
    for _ in 0..left_offset + tree_width/2 - 2 {
        print!(" ");
    }

    println!("{}", "*   *".yellow().blink());
    for _ in 0..left_offset + tree_width/2 - 2 {
        print!(" ");
    }
    println!("{}","  *  ".yellow().blink());
    for _ in 0..left_offset + tree_width/2 - 2 {
        print!(" ");
    }
    println!("{}","*   *".yellow().blink());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing tree height...");
        return;
    }
    let tree_height: u8 = args[1].parse().unwrap();
    let tree_width:u8 = tree_height * 2;  //final width is the final offset tree_height*2 - 1
    let left_offset = 50;
    loop {
        sleep(Duration::from_millis(500));
        clear().expect("Failed to clean screen...");
        print!("\n\n\n");
        draw_star(tree_width, left_offset);
        draw_tree(tree_height, tree_width, left_offset);
        draw_trunk(tree_height, tree_width, left_offset);       
        print!("\n\n\n\n");
        unsafe { N += 1 };
    }
}
