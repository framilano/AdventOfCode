use std::fs;

fn main() {
    let file: String = fs::read_to_string("../input").expect("Should contain file");

    //Split String with new line as separator, creating a vector
    let arr: Vec<&str> = file.rsplit("\n").collect();

    let floor_line = arr[0].chars();

    let mut floor: i32 = 0;
    let mut index:u32 = 1;
    for char in floor_line {
        if char == '(' {floor+=1}
        if char == ')' {floor-=1}

        if floor == -1 {
            break;
        }
        index += 1;
    }

    println!("Basement position: {}", index);
}
