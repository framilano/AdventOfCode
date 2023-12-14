use std::fs;

fn get_wrapping_paper(line: &str) -> u32{
    let dims: Vec<&str> = line.rsplit('x').collect();
    let length: u32 = dims[0].parse().unwrap();
    let width: u32 = dims[1].parse().unwrap();
    let height: u32 = dims[2].parse().unwrap();

    let mut prod_arr: Vec<u32> = Vec::new();
    prod_arr.push(length * width);
    prod_arr.push(length * height);
    prod_arr.push(height * width);

    let min_area = prod_arr.iter().min().unwrap();

    return min_area + 2*prod_arr[0] + 2*prod_arr[1] + 2*prod_arr[2];
}


fn main() {
    let file: String = fs::read_to_string("../input").expect("Should contain file");

    //Split String with new line as separator, creating a vector
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();

    let mut sum:u32 = 0;
    for line in arr {
        if line.is_empty() {continue;}
        sum += get_wrapping_paper(line)
    }

    println!("Total amount of wrapping paper: {}", sum);

}
