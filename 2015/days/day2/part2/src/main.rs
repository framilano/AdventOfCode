use std::fs;

fn get_total_ribbon_length(line: &str) -> u32{
    let dims: Vec<&str> = line.rsplit('x').rev().collect();
    let mut int_dims: Vec<u32> = dims.iter().map(|x| x.parse().unwrap()).collect();
    
    int_dims.sort();

    let partial_ribbon = 2* int_dims[0] + 2*int_dims[1];
    let bow_ribbon = int_dims[0] * int_dims[1] * int_dims[2];

    return partial_ribbon + bow_ribbon;
}


fn main() {
    let file: String = fs::read_to_string("../input").expect("Should contain file");

    //Split String with new line as separator, creating a vector
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut sum:u32 = 0;
    for line in arr {
        if line.is_empty() {continue;}
        sum += get_total_ribbon_length(line)
    }

    println!("Total feet of ribbon: {}", sum);

}
