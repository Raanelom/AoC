use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_chars};
use std::collections::HashSet;

#[derive(Debug,Clone)]
struct OctopusGrid {
    width: usize,
    height: usize,
    octopuses: Vec<Octopus>
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Octopus {
    index: usize,
    energy: usize
}

impl OctopusGrid {
    fn init() -> OctopusGrid {
        OctopusGrid {
            width: 0,
            height: 0,
            octopuses: Vec::new()
        }
    }

    fn read_input(&mut self, lines: &mut std::iter::Peekable<std::str::Lines>) {
        if let Some(line) = lines.peek() {
            self.width = line.len();
        }
        for line in lines {
            let mut new_octopuses: Vec<Octopus> = split_chars(&line.to_string())
                .filter(|x| x.is_digit(10))
                .enumerate()
                .map(|(i,x)| Octopus::new(x.to_digit(10).unwrap() as usize, i + (self.height*self.width)))
                .collect::<Vec<Octopus>>();
            self.octopuses.append(&mut new_octopuses);
            self.height += 1;
        }
    }

    fn get_neighbour(&self, has_neighbour: bool, neighbour_index: usize) -> Option<&Octopus> {
        if has_neighbour {
            return Some(&self.octopuses[neighbour_index]);
        }
        return None
    }
    fn has_left_neighbour(&self, index: usize) -> bool {
        return index % self.width > 0;
    }
    fn has_top_neighbour(&self, index: usize) -> bool {
        return index % self.width != index;
    }
    fn has_right_neighbour(&self, index: usize) -> bool {
        return (index + 1) % self.width > 0;
    }
    fn has_bottom_neighbour(&self, index: usize) -> bool {
        return index + self.width < self.width*self.height
    }

    fn left_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_left_neighbour(index), index - std::cmp::min(1, index));
    }
    fn top_left_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_left_neighbour(index) && self.has_top_neighbour(index), 
            index - std::cmp::min(1 + self.width, index));
    }
    fn top_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_top_neighbour(index), index - std::cmp::min(self.width, index));
    }
    fn top_right_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_top_neighbour(index) && self.has_right_neighbour(index), 
            index - std::cmp::min(self.width, index) + 1);
    }
    fn right_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_right_neighbour(index), index + 1);
    }
    fn bottom_right_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_right_neighbour(index) && self.has_bottom_neighbour(index), 
            index + 1 + self.width);
    }
    fn bottom_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_bottom_neighbour(index), index + self.width);
    }
    fn bottom_left_neighbour(&self, index: usize) -> Option<&Octopus> {
        return self.get_neighbour(self.has_bottom_neighbour(index) && self.has_left_neighbour(index), 
            index + self.width - 1);
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        let neighbours: Vec<Option<&Octopus>> = vec![self.left_neighbour(index), self.top_left_neighbour(index),
            self.top_neighbour(index), self.top_right_neighbour(index), self.right_neighbour(index),
            self.bottom_right_neighbour(index), self.bottom_neighbour(index),
            self.bottom_left_neighbour(index)];

        neighbours
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().index)
            .collect::<Vec<usize>>()
    }
}

impl Octopus {
    fn new(energy: usize, index: usize) -> Octopus {
        Octopus {
            energy: energy,
            index: index
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let mut lines = split_lines(&input);

    let mut octopus_grid = OctopusGrid::init();
    octopus_grid.read_input(&mut lines);

    println!("{:?}", octopus_grid.neighbours(16));
    
}