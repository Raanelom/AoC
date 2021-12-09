use std::env;
use util::input_operations::{read_file_to_string};

fn determine_lowest_cost(start: &usize, end: &usize, horizontal_positions: &Vec<usize>) -> usize {
    let position = start + ((end - start) / 2);
    let costs = determine_costs(horizontal_positions, &position);
    let right_neighbour_costs = determine_costs(horizontal_positions, &(position + 1));
    if end - start == 1 {
        println!("start: {}", start);
        println!("end: {}", end);
        println!("position: {}", position);
        return std::cmp::min(costs, right_neighbour_costs);
    }
    if right_neighbour_costs < costs {
        println!("go bigger previous_costs <= costs\n");
        // go for the bigger part
        determine_lowest_cost(&position, end, horizontal_positions)
    }
    else {
        println!("go smaller previous_costs <= costs\n");
        // go for the smaller part
        determine_lowest_cost(start, &position, horizontal_positions)
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let mut horizontal_positions: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    horizontal_positions.sort();

    let min = horizontal_positions.iter().min().unwrap();
    let max = horizontal_positions.iter().max().unwrap();

    let lowest_cost = determine_lowest_cost(&min, &max, &horizontal_positions);
    println!("Lowest cost: {}", lowest_cost);
}

fn determine_costs(horizontal_positions: &Vec<usize>, suggested_position: &usize) -> usize {
    let mut fuel: usize = 0;
    for position in horizontal_positions {
        fuel += compute_step_costs(std::cmp::max(position, suggested_position) - std::cmp::min(position, suggested_position));
    }
    fuel
}

fn compute_step_costs(diff: usize) -> usize {
    let mut costs = 0;
    for i in 1..=diff {
        costs += i;
    }
    costs
}