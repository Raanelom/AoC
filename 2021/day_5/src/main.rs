use std::env;
use util::input_operations::{read_file_to_string,split_double_newlines,split_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let lines = split_lines(&input);
    let coordinates = lines.map(|x| x.split(" -> ")
        .map(|y| y.split(",")
            .map(|z| z.parse::<usize>().unwrap())
                .collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>())
        .collect::<Vec<Vec<Vec<usize>>>>();

    println!("{:?}", coordinates)

    // TODO:
    // 1. Filter non-line-elements
    // 2. For each line, determine the direction
    // 3. For each line, determine the length
}