use std::env;
use util::input_operations::{read_file_to_string,split_lines};
use std::collections::LinkedList;

struct Lanternfish {
    internal_timer: u8,
    children: LinkedList<Lanternfish>
}

impl Lanternfish {
    fn new(internal_timer: u8) -> Lanternfish {
        Lanternfish {
            internal_timer: internal_timer,
            children: LinkedList::new()
        }
    }

    fn next_day(&mut self) {
        for child in self.children.iter_mut() {
            child.next_day();
        }
        if self.internal_timer == 0 {
            self.children.push_back(Lanternfish::new(8));
            self.internal_timer = 6;
        } 
        else {
            self.internal_timer = self.internal_timer - 1;
        }
    }

    fn family_size(&self) -> usize {
        let mut size = 1; // Start with counting yourself
        for child in self.children.iter() {
            size += child.family_size();
        }
        size
    }

    // fn determine_children(days: usize) {
    //     let next_child = self.internal_timer;
    //     let children_remaining = days - next_child 
    // }
}

fn make_children(days: usize, lanternfish_family: &mut Vec<Lanternfish>) -> usize {
    for _i in 0..days {
        for lanternfish in lanternfish_family.iter_mut() {
            lanternfish.next_day();
        }
    }
    lanternfish_family
        .iter()
        .map(|l| l.family_size())
        .sum::<usize>()
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

    let family_size = make_children(256, &mut lanternfish_family);
    println!("{}", family_size);
}
