use std::env;
use util::input_operations::{read_file_to_string};
use std::collections::HashMap;

struct Lanternfish {
    internal_timer: u8
}

impl Lanternfish {
    fn new(internal_timer: u8) -> Lanternfish {
        Lanternfish {
            internal_timer: internal_timer
        }
    }

    fn determine_more_children(lanternfish_prediction: &mut HashMap<usize, usize>, days: usize) -> usize {
        if lanternfish_prediction.contains_key(&days) {
            return *lanternfish_prediction.get(&days).unwrap();
        }
        let mut children_remaining = 1 + (days / 7);
        let mut remaining_days = days;
        while remaining_days >= 9 {
            children_remaining += Lanternfish::determine_more_children(lanternfish_prediction, remaining_days - 9);
            remaining_days -= 7;
        }
        lanternfish_prediction.insert(days, children_remaining);
        children_remaining
    }

    fn determine_children(&self, lanternfish_prediction: &mut HashMap<usize, usize>, days: usize) -> usize {
        // Correct the number of days (1-indexed instead of 0-indexed)
        let days = days - 1;
        let next_child = usize::from(self.internal_timer);
        if days < next_child {
            return 0;
        }
        let mut remaining_days = days - next_child;
        let mut children_remaining = 1 + (remaining_days / 7);
        while remaining_days >= 9 {
            children_remaining += Lanternfish::determine_more_children(lanternfish_prediction, remaining_days - 9);
            remaining_days -= 7;
        }
        return children_remaining;
        
    }
}

fn count_family(days: usize, lanternfish_family: &mut Vec<Lanternfish>) -> usize {
    let mut latnernfish_prediction = HashMap::<usize, usize>::new();
    let lanternfish_family_size = lanternfish_family
        .iter()
        .map(|l| l.determine_children(&mut latnernfish_prediction, days))
        .sum::<usize>() 
        + lanternfish_family.iter().count();
    println!("{}", lanternfish_family_size);
    lanternfish_family_size
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);

    let mut lanternfish_family: Vec<Lanternfish> = input
        .split(",")
        .map(|l| Lanternfish::new(l.parse().unwrap()))
        .collect();

    count_family(256, &mut lanternfish_family);
}
