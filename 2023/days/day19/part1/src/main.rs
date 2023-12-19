use std::collections::HashMap;
use std::process::exit;
use std::fs;

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

fn get_input_arr() -> Vec<String> {
    let file = fs::read_to_string("../input");
    if file.is_err() {
        println!("File not found...");
        exit(0);
    }
    let mut arr: Vec<String> = file.ok().unwrap().rsplit("\n").map(|x| String::from(x)).collect();
    arr.reverse();
    //Removing last empty line
    arr.pop();
    
    return arr;
}

fn parse_input(arr_str: &Vec<&str>) -> (HashMap<String, Vec<String>>, Vec<Part>) {
    let mut parts: Vec<Part> = Vec::new();
    let mut workflows: HashMap<String, Vec<String>> = HashMap::new();

    let mut adding_workflows = true;

    for line in arr_str {
        if line.is_empty() {
            adding_workflows = !adding_workflows;
            continue;
        }
       
        if adding_workflows {
            let brackets_split: Vec<&str> = line.split("{").collect();
            let name = brackets_split[0].to_string();
            let conditions_split: Vec<&str> = brackets_split[1].split(",").collect();
            let mut conditions: Vec<String> = conditions_split.iter().map(|x| x.to_string()).collect();
            let length = conditions.len()-1;
            conditions[length] = conditions.last().unwrap().replace("}", "");

            workflows.insert(name, conditions);
        } else {
            let mut string_line: String = line.to_string();
            string_line.pop();
            string_line.remove(0);

            let line_split: Vec<&str> = string_line.split(",").collect();
            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;

            for value in line_split {
                if value.contains("x=") {
                    x = value.replace("x=", "").parse::<u32>().unwrap();
                } else if value.contains("m=") {
                    m = value.replace("m=", "").parse::<u32>().unwrap();
                } else if value.contains("a=") {
                    a = value.replace("a=", "").parse::<u32>().unwrap();
                } else if value.contains("s=") {
                    s = value.replace("s=", "").parse::<u32>().unwrap();
                }
            }

            parts.push(Part { x, m, a, s })
        }
    }

    

    return (workflows, parts);
}

fn is_condition_true(condition: &str, part: &Part) -> bool {
    if condition.contains(">") {
        let condition_split: Vec<&str> = condition.split(">").collect();
        if condition_split[0] == "x" {
            if part.x > condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "m" {
            if part.m > condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "a" {
            if part.a > condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "s" {
            if part.s > condition_split[1].parse().unwrap() {return true}
            else {return false}
        }
    }

    if condition.contains("<") {
        let condition_split: Vec<&str> = condition.split("<").collect();
        if condition_split[0] == "x" {
            if part.x < condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "m" {
            if part.m < condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "a" {
            if part.a < condition_split[1].parse().unwrap() {return true}
            else {return false}
        } else if condition_split[0] == "s" {
            if part.s < condition_split[1].parse().unwrap() {return true}
            else {return false}
        }
    }

    return false;

}

fn is_accepted(part: &Part, workflows: &HashMap<String, Vec<String>>, flow_name: &str) -> bool {
    if flow_name == "A" {return true}
    if flow_name == "R" {return false}

    let conditions = workflows.get(flow_name).unwrap();

    for condition in conditions {
        if condition == "A" {
            return true;
        }
        else if condition == "R" {
            return false;
        }
        else if condition.contains(":") {
            let condition_split: Vec<&str> = condition.split(":").collect();
            let destination_if_true = condition_split[1];
            if is_condition_true(condition_split[0], part) {
                if is_accepted(part, workflows, destination_if_true) {
                    return true;
                } else {return false}
            }
        } else {
            if is_accepted(part, workflows, condition) {
                return true;
            } else {return false}
        }
    }

    return false;
}

fn main() {
        //Input parsing
        let arr: Vec<String> = get_input_arr();
        let arr_str: Vec<&str> = arr.iter().map(|x| x.as_str()).collect();

        let (workflows, parts): (HashMap<String, Vec<String>>, Vec<Part>) = parse_input(&arr_str);

        let mut sum_of_accepted_values = 0;

        for part in parts {
            let status = is_accepted(&part, &workflows, "in");
            if status {
                sum_of_accepted_values += part.x + part.m + part.a + part.s;
            }
        }
        println!("Sum of accepted values: {}", sum_of_accepted_values);
}
