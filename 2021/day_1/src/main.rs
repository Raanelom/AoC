use std::env;
use std::fs;

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

fn to_int(o: &Option<&str>) -> u32 {
    o.unwrap().parse().unwrap()
}

fn calculate_measurements_sliding_window(sequential_depths: String, window_size: u32) {
    let mut sequential_depths_iterator = sequential_depths.split_whitespace().peekable();
    let mut increasing_depth_counter = 0;
    let mut depths_window = Vec::new();
    //sequential_depths_iterator.next();
    for _i in 0..window_size {
        if !sequential_depths_iterator.peek().is_some() {
            panic!("There are insufficient items available for measuring the depth");
        }
        depths_window.push(to_int(&sequential_depths_iterator.next()));
    }
    // Keep iterating until the iterator is empty
    while sequential_depths_iterator.peek().is_some() {
        let previous_depths_sum: u32 = depths_window.iter().sum();
        // Remove first added item
        depths_window.remove(0);
        depths_window.push(to_int(&sequential_depths_iterator.next()));
        let current_depths_sum: u32 = depths_window.iter().sum();
        
        increasing_depth_counter = increasing_depth_counter + (current_depths_sum > previous_depths_sum) as u32;        
    }
    println!("Depth increased {} times", increasing_depth_counter);
}

fn main() {
    let sequential_depths = read_file_to_string();

    calculate_measurements_sliding_window(sequential_depths, 3);
}