use regex::Regex;
use std::fs;

fn parse_input() -> Vec<String> {
    let filename = "/Users/corymcdonald/repos/advent/day4/src/day4.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut input: Vec<String> = Vec::new();

    for item in contents.split("\n\n") {
        if item == "" {
            break;
        }
        input.push(String::from(item))
    }
    return input;
}

fn part1(input: Vec<String>) -> i32 {
    let fields = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    let mut valid_passports = 0;
    for value in input.iter() {
        let mut is_valid = true;
        for field in fields.iter() {
            if !value.contains(field) {
                is_valid = false;
            }
        }
        if is_valid {
            valid_passports += 1;
        }
    }
    valid_passports
}
fn validate_height(value: &str) -> bool {
    let mut height_valid: bool = Regex::new(r"\d(in|cm)").unwrap().is_match(value);
    if height_valid {
        let height = value
            .replace("cm", "")
            .replace("in", "")
            .parse()
            .unwrap_or(0);
        if value.contains("cm") {
            height_valid = height >= 150 && height <= 193
        } else {
            height_valid = height >= 59 && height <= 76
        }
    }
    height_valid
}

fn part2(input: Vec<String>) -> i32 {
    const FIELDS: [&str; 7]= ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
    const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    const HEX_REGEX: &str = r"^#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$";
    const PID_REGEX: &str = r"^[0-9]{9}$";

    let mut valid_passports = 0;

    for value in input.iter() {
        let mut is_valid = true;
        for field in FIELDS.iter() {
            if !value.contains(field) {
                is_valid = false;
            }
        }
        if !is_valid {
            continue;
        }

        // Struggled with parsing and having all the strings on one line.
        let one_line = value.replace("\n", " ");
        let split_fields: Vec<&str> = one_line.split(" ").collect();

        for property in split_fields.iter() {
            if property.is_empty() {
                break;
            }
            let separated: Vec<&str> = property.split(':').collect();
            let field_value: &str = &separated[1];
            let number: i32 = field_value.parse().unwrap_or(0);

            let field_valid = match separated[0] {
                "byr" => number >= 1920 && number <= 2002,
                "iyr" => number >= 2010 && number <= 2020,
                "eyr" => number >= 2020 && number <= 2030,
                "hgt" => validate_height(field_value),
                "hcl" => Regex::new(HEX_REGEX).unwrap().is_match(field_value),
                "ecl" => EYE_COLORS.contains(&field_value),
                "pid" => Regex::new(PID_REGEX).unwrap().is_match(field_value),
                _ => true,
            };

            if !field_valid {
                is_valid = false
            }
        }

        if is_valid {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn main() {
    // Have to call parse_input() twice because a resource can only have one owner
    // https://doc.rust-lang.org/rust-by-example/scope/move.html
    println!("Part 1 Valid passports {}", part1(parse_input()));
    println!("Part 2 Valid passports {}", part2(parse_input()));
}
