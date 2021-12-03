use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_whitespace};

enum Direction {
    Up,
    Down,
    Forward
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let positions = read_file_to_string(&args[1]);

    determine_position(positions);
}

fn get_direction_and_speed(position: std::string::String) -> (std::string::String, i32) {
    let mut position_split = split_whitespace(&position);
    let direction = position_split.next().unwrap().to_string();
    let speed: i32 = position_split.next().unwrap().parse().unwrap();
    return (direction, speed)
}

fn determine_position(file_result: String) {
    let mut depth = 0;
    let mut horizontal_position = 0;
    for position in split_lines(&file_result) {
        let (direction, speed) = get_direction_and_speed(position.to_string());
        match direction.as_ref() {
            "forward" => horizontal_position += speed,
            "up" => depth -= speed,
            "down" => depth += speed,
            _ => panic!("Invalid direction {}. It should be one of the following: 'forward', 'up', 'down'", direction)
        }
    }
    println!("{}", depth);
    println!("{}", horizontal_position);
    println!("The amazing multiplier returns {}", depth * horizontal_position);
}
