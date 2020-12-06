use std::fs;

fn get_input() -> Vec<String> {
  let filename = "day2.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let mut input: Vec<String> = Vec::new();

  for item in contents.split('\n') {
      if item == "" {
          break;
      }
      input.push(String::from(item))
  }
  return input;
}


fn main() {
  let input = get_input();


  // let mut input: Vec<String> = Vec::new();
  // input.push(String::from("1-3 a: abcde"));
  // input.push(String::from("1-3 b: cdefg"));
  // input.push(String::from("2-9 c: ccccccccc"));
  part2(input)
}

fn part1(input : Vec<String>) {
  let mut valid = 0;

  for value in input.iter() {
    let source : Vec<&str> =  value.split(" ").collect();

    let numbers : Vec<&str>  = source[0].split("-").collect();

    let min : i32 = numbers[0].parse().unwrap();
    let max :i32 = numbers[1].parse().unwrap();

    let first_char = source[1].chars().next().unwrap();

    let mut index = 0;
    for character in source[2].chars() {
      if character == first_char {
        index += 1;
      }
    }

    if min <= index && index <= max {
      valid += 1;
    }
  }
  println!("Number of valid {}", valid)
}

fn part2(input : Vec<String>) {
  let mut valid = 0;

  for value in input.iter() {
    let source : Vec<&str> =  value.split(" ").collect();

    let numbers : Vec<&str>  = source[0].split("-").collect();

    let min : usize = numbers[0].parse().unwrap();
    let max : usize = numbers[1].parse().unwrap();


    let first_char = source[1].chars().next().unwrap();

    let mut characters = source[2].as_bytes();

    let first_character = characters[min -1] as char;

    let second_character = characters[max - 1] as char;

    if (first_character == first_char && second_character != first_char) || (first_character != first_char && second_character == first_char) {
      valid += 1;
    }

  }
  println!("Number of valid {}", valid)
}
