use std::env;
use std::fs;

//const PUZZLE_INPUT: &str = "input";

fn read_file_to_string() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let filename = &args[1];
    println!("In file {}", filename);

    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

fn calculate_measurements(sequential_depths: String) {
    let mut sequential_depths_iterator = sequential_depths.split_whitespace();
    let mut increasing_depth_counter = 0;
    let mut previous_depth: u32 = sequential_depths_iterator
        .next().unwrap()
        .parse().unwrap();
    for depth in sequential_depths_iterator {
        println!("{}", depth);
        let current_depth: u32 = depth.parse().unwrap();
        increasing_depth_counter = increasing_depth_counter + (current_depth > previous_depth) as u32;
        previous_depth = current_depth;
    }
    println!("Depth increased {} times", increasing_depth_counter);
}

fn main() {
    let sequential_depths = read_file_to_string();

    calculate_measurements(sequential_depths);

    //println!("With text:\n{}", contents);
}