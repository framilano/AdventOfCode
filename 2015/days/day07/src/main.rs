use std::{fs, collections::HashMap};

//Given a token, that can contain a number or a wire identifier
//Return the value for values_map[wire_id] if wire_id is a str, return the value immediately if it's already a number
//If the wire id is a str but it's not available in values__map return Option::None
fn parse_value_from_context(values_map: &mut HashMap<&str, u16>, token: &str) -> Option<u16> {
    if token.parse::<u16>().is_ok() {
        return Option::Some(token.parse::<u16>().unwrap());
    } else {
        if values_map.get(token).is_some() {
            return Option::Some(*values_map.get(token).unwrap());
        }
    }

    return Option::None;
}

fn and_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: &Vec<&'a str>) {
    let left_opt = parse_value_from_context(values_map, tokens[0]);
    let right_opt = parse_value_from_context(values_map, tokens[2]);

    if left_opt.is_none() || right_opt.is_none() {return;}

    values_map.insert(tokens[4], left_opt.unwrap() & right_opt.unwrap());
}

fn or_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: &Vec<&'a str>) {
    let left_opt = parse_value_from_context(values_map, tokens[0]);
    let right_opt = parse_value_from_context(values_map, tokens[2]);

    if left_opt.is_none() || right_opt.is_none() {return;}

    values_map.insert(tokens[4], left_opt.unwrap() | right_opt.unwrap());
}

fn not_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: &Vec<&'a str>) {
    let symbol_opt = parse_value_from_context(values_map, tokens[1]);

    if symbol_opt.is_none() {return;}

    values_map.insert(tokens[3], !symbol_opt.unwrap());
}

fn lshift_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: &Vec<&'a str>) {
    let number_to_shift_opt = values_map.get(tokens[0]);
    if number_to_shift_opt.is_none() {
        return;
    } else {
        let numbers_to_shift = number_to_shift_opt.unwrap();
        values_map.insert(tokens[4], numbers_to_shift << tokens[2].parse::<u16>().unwrap());
    }
}

fn rshift_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: &Vec<&'a str>) {
    let number_to_shift_opt = values_map.get(tokens[0]);
    if number_to_shift_opt.is_none() {
        return;
    } else {
        let numbers_to_shift = number_to_shift_opt.unwrap();
        values_map.insert(tokens[4], numbers_to_shift >> tokens[2].parse::<u16>().unwrap());
    }
}

fn assign_case<'a>(values_map: &mut HashMap<&'a str, u16>, tokens: Vec<&'a str>) {
    let assign_opt = parse_value_from_context(values_map, tokens[0]);
    if assign_opt.is_none() {return;}
    values_map.insert(tokens[2], assign_opt.unwrap());
}

fn get_value_for_a(arr: Vec<&str>) -> u16 {
    let mut values_map: HashMap<&str, u16> = HashMap::new();
    
    //Until the values_map with key 'a' is Some, we keep rewiring the cables using the new info got on the previous loop on values_map
    loop {
        for line in &arr {
            let tokens: Vec<&str> = line.split(" ").collect();
            
            if tokens.contains(&"AND") {
                and_case(&mut values_map, &tokens)
            } else if tokens.contains(&"OR") {
                or_case(&mut values_map, &tokens)
            } else if tokens.contains(&"NOT") {
                not_case(&mut values_map, &tokens)
            } else if tokens.contains(&"LSHIFT") {
                lshift_case(&mut values_map, &tokens)
            } else if tokens.contains(&"RSHIFT") {
                rshift_case(&mut values_map, &tokens)
            } else {
                assign_case(&mut values_map, tokens)
            }
        }
        if values_map.get("a").is_none() {continue;}
        else {
            return values_map.get("a").unwrap().clone();
        }
    }

}

fn main() {
    let file = fs::read_to_string("../input").expect("Should read file...");

    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    arr.reverse();
    arr.pop();

    let a: u16 = get_value_for_a(arr);


    //Result is 16076 for part1, just change 19318 -> b to 16076 -> b to get the result for part2
    //No code changes required

    println!("'a' value: {}", a);
}
