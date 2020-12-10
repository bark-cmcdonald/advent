fn parse_input() -> Vec<String> {
    let filename = "/Users/corymcdonald/repos/advent/day8/src/day8.txt";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input: Vec<String> = Vec::new();

    for item in contents.split("\n") {
        if item == "" {
            break;
        }
        input.push(String::from(item))
    }
    return input;
}

fn part1(input: Vec<String>) -> i32 {
    let mut accumulator = 0;
    let mut seen: Vec<i32> = Vec::new();

    let mut i: i32 = 0;

    while !seen.contains(&i) {
        let commands: Vec<&str> = input[i as usize].split(" ").collect();
        let value: i32 = commands[1].parse().unwrap_or(0);
        seen.push(i);

        match commands[0] {
            "acc" => {
                accumulator += value;
            }
            "jmp" => {
                i += value;
            }
            _ => {}
        }

        if !commands[0].contains("jmp") {
            i += 1;
        }
    }
    accumulator
}

fn part2(input: Vec<String>) -> i32 {
    for (index, value) in input.iter().enumerate() {
        let mut result = false;
        let mut accumulator = 0;
        if value.contains("nop") {
            let mut copy = input.to_vec();
            copy[index] = copy[index].replace("nop", "jmp");
            let (a, b) = part2_do(copy);
            accumulator = a;
            result = b
        } else if value.contains("jmp") {
            let mut copy = input.to_vec();
            copy[index] = copy[index].replace("jmp", "nop");
            let (a, b) = part2_do(copy);
            accumulator = a;
            result = b

        }
        if result {
            return accumulator;
        }
    }
    return 0;
}
fn part2_do(input: Vec<String>) -> (i32, bool) {
    let mut accumulator = 0;
    let mut seen: Vec<i32> = Vec::new();

    let mut i: i32 = 0;

    while !seen.contains(&i) && (i as usize) < input.len() {
        let commands: Vec<&str> = input[i as usize].split(" ").collect();
        let value: i32 = commands[1].parse().unwrap_or(0);
        seen.push(i);

        match commands[0] {
            "acc" => {
                accumulator += value;
            }
            "jmp" => {
                i += value;
            }
            _ => {}
        }

        if !commands[0].contains("jmp") {
            i += 1;
        }
    }
    if (i as usize) >= input.len() {
        return (accumulator, true);
    }
    (accumulator, false)
}

fn main() {
    let input = parse_input();
    part1(input.to_vec());
    part2(input.to_vec());
}
