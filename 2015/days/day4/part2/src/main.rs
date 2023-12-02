use md5;

const INPUT: &str = "bgvyzdsv";

fn main() {
    
    let mut index = 0;

    loop {
        let string_to_hash = INPUT.to_string() + &index.to_string();
        let digest = md5::compute(string_to_hash.as_bytes());
        let digest_string = format!("{:x}", digest);
        if digest_string.starts_with("000000") {
            println!("{}", index);
            return;
        }
        index += 1
    }
}
