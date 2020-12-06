use std::collections::HashMap;

fn parse_input() -> Vec<String> {
    let filename = "/Users/corymcdonald/repos/advent/day6/src/day6.txt";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input: Vec<String> = Vec::new();

    for item in contents.split("\n\n") {
        if item == "" {
            break;
        }
        input.push(String::from(item))
    }
    return input;
}

fn part1(input: Vec<String>) -> usize {
    let mut sum = 0;
    for value in input.iter() {
        let lines: Vec<&str> = value.split("\n").collect();

        let mut alphabet = HashMap::new();

        for line in lines.iter() {
            for character in line.chars() {
                alphabet.insert(character, true);
            }
        }

        sum += alphabet.keys().count();
    }
    return sum;
}


fn part2(input: Vec<String>) -> usize {
    let mut sum = 0;
    for value in input.iter() {
        let lines: Vec<&str> = value.split("\n").collect();

        let mut temp = 0;
        let mut alphabet = HashMap::new();

        for line in lines.iter() {
            for character in line.chars() {
                let entry = alphabet.entry(character).or_insert(0);
                *entry += 1
            }
        }

        for key in alphabet.keys() {
            let count = alphabet.get(key).unwrap_or(&0);
            if *count == lines.len() {
                temp += 1
            }
        }

        sum += temp
    }
    return sum;
}


fn main() {

    println!("{}", part1(parse_input()));
    println!("{}", part2(parse_input()));
}
