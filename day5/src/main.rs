fn parse_input() -> Vec<String> {
    let filename = "/Users/corymcdonald/repos/advent/day5/src/day5.txt";
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

fn find_row(chars: Vec<char>, start: i32, end: i32) -> i32 {
    if start == end {
        return start;
    }

    let mut copy = chars;
    let first_character = copy.remove(0);
    let midpoint = ((start + end) as f32) / 2f32;

    match first_character {
        'F' | 'L' => {
            return find_row(copy, start, midpoint.floor() as i32);
        }
        'B' | 'R' => {
            return find_row(copy, midpoint.ceil() as i32, end);
        }
        _ => {}
    }

    return 0;
}

fn main() {
    let mut ids: Vec<i32> = Vec::new();
    for input in parse_input().iter() {
        let row_chars = input.chars().take(7).collect();
        let column_chars = input[7..].chars().take(3).collect();

        let row = find_row(row_chars, 0, 127);
        let column = find_row(column_chars, 0, 7);
        let id = row * 8 + column;
        ids.push(id);
    }
    // Part 1
    ids.sort();
    let highest = *ids.last().unwrap_or(&0);

    // Part 2
    let starting_seat = ids.first().unwrap_or(&0);
    let mut missing = 0;

    for i in 0..ids.len() {
        let seat = i as i32 + starting_seat;

        if ids[i] != seat {
            missing = seat;
            break;
        }
    }

    println!("highest {}", highest);
    println!("missing {}", missing);
}
