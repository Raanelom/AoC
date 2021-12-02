use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_whitespace};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let positions = read_file_to_string(&args[1]);

    determine_position(positions);
}

fn determine_position(file_result: String) {
    for position in split_lines(&file_result) {
        for position in split_whitespace(&String::from(position)) {
            println!("{}", position);
        }
    }
}
