use std::fs;

/**
 * Find the max value of the sum of consecutive integers in input.txt
 * Let's say
 * "
 * 
 * 1
 * 2
 * 3
 * 
 * 3
 * 4
 * 
 * "
 * 
 * The max value is 7 (3+4)
 * 
 */

fn trim_array(arr: &mut Vec<&str>) {
    //Removing new lines at start and end
    let mut i: usize = 0;
    loop {
        if arr[i] == "" {
            arr.remove(i);
            i = 0;
        } else {
            break
        }
    }

    i = arr.len() - 1;

    loop {
        if arr[i] == "" {
            arr.remove(i);
            i = arr.len() - 1;
        } else {
            break;
        }
    }
}

fn main() {
    //Print hello world
    println!("Hello, world!");

    //Declaring variable
    let x = 10;
    println!("{}", x);
    
    let file: String = fs::read_to_string("src/input.txt").expect("Should contain file");
    println!("{}", file);

    //Split String with new line as separator, creating a vector
    let mut arr: Vec<&str> = file.rsplit("\n").collect();
    
    //Putting it in the right order, file.rsplit("\n").collect(); collects it in reverse
    arr.reverse();
    
    println!("Original Array form: \n{:?}", arr);
    
    //Passing variable to function as reference, we call the action of creating a reference borrowing.
    trim_array(&mut arr);

    //Passing variable to function as value
    // trim_array(arr)
    // But arr doesn't implement the Copy method, so we need to pass a clone of it
    //trim_array(arr.clone())

    println!("Array form after trimming: \n{:?}", arr);

    //Using a sum vector to contain all sums
    let mut sum_vector: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut added_flag = false;

    //Using this logic it is not necessary to trim the array, just doing it for fun
    for i in 0..arr.len() {
        if arr[i] == "" && added_flag {
            sum_vector.push(sum);
            sum = 0;
            added_flag = false;
            continue;
        } 

        if arr[i] != "" {
            added_flag = true;
            sum += arr[i].parse::<i32>().unwrap();

            if i == arr.len() - 1 { 
                sum_vector.push(sum);
            }
        }
    }

    println!("Sums Vector\n{:?}", sum_vector);

    //Iteration over iterator, without using any Collection
    println!("Number in sums_vector being iterated without Collections");
    let mut sum_vector_iter = sum_vector.iter();
    loop {
        let elem = sum_vector_iter.next();
        if elem.is_none() {
            break;
        }
        println!("{}", elem.unwrap());
    }
    
    let max = sum_vector.iter().max().unwrap();

    println!("\nMax value found {}", max);
}
