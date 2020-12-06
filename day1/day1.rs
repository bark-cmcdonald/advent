// use std::env;
use std::fs;

// This is the main function
fn main() {
    let filename = "day1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input: Vec<i32> = Vec::new();

    for item in contents.split('\n') {
        if item == "" {
            break;
        }
        let integer = item.parse().unwrap();
        input.push(integer)
    }

    let year = 2020;

    let mut found = false;
    for (i, value) in input.iter().enumerate() {
        let sum = year - value;
        let slice = &input[i..input.len()];

        for value_2 in slice.iter() {
            let y_sum = sum - y;

            if input.contains(&y_sum) {
                let result = value * y_sum * y;
                println!("The result is {} + {} + {} = 2020, x * y * z = {}", value, y_sum, y, result);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }
}
