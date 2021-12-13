use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_chars};

#[derive(Debug,Clone)]
struct HeightMap {
    map_width: usize,
    map_height: usize,
    locations: Vec<Location>
}

#[derive(Debug,Clone)]
struct Location {
    height: usize
}

impl HeightMap {
    fn init() -> HeightMap {
        HeightMap {
            map_width: 0,
            map_height: 0,
            locations: Vec::new()
        }
    }

    fn read_input(&mut self, lines: &mut std::iter::Peekable<std::str::Lines>) {
        if let Some(line) = lines.peek() {
            self.map_width = line.len();
        }
        for line in lines {
            self.map_height += 1;
            let mut new_locations: Vec<Location> = split_chars(&line.to_string())
                .filter(|x| x.is_digit(10))
                .map(|x| Location::new(x.to_digit(10).unwrap() as usize))
                .collect::<Vec<Location>>();
            self.locations.append(&mut new_locations);
        }
    }

    fn get_location(&self, index: usize) -> &Location {
        assert!(self.map_width > 0);
        assert!(index < self.map_width*self.map_height);
        return &self.locations[(index / self.map_width) + (index % self.map_width)]
    }

    fn left_neighbour(&self, index: usize) -> Option<&Location> {
        if index % self.map_width == 0 {
            return None
        }
        return Some(&self.locations[index - 1]);
    }

    fn top_neighbour(&self, index: usize) -> Option<&Location> {
        if index % self.map_width == index {
            return None
        }
        return Some(&self.locations[index - self.map_width]);
    }

    fn right_neighbour(&self, index: usize) -> Option<&Location> {
        if (index + 1) % self.map_width == 0 {
            return None
        }
        return Some(&self.locations[index + 1]);
    }

    fn bottom_neighbour(&self, index: usize) -> Option<&Location> {
        if index + self.map_width >= self.map_width*self.map_height {
            return None
        }
        return Some(&self.locations[index + self.map_width]);
    }

    fn is_lowest_point(&self, index: usize) -> bool {
        let current_height = self.locations[index].height;
        let mut is_lowest = true;
        if let Some(left) = self.left_neighbour(index) {
            is_lowest &= current_height < left.height;
        }
        if let Some(top) = self.top_neighbour(index) {
            is_lowest &= current_height < top.height;
        }
        if let Some(right) = self.right_neighbour(index) {
            is_lowest &= current_height < right.height;
        }
        if let Some(bottom) = self.bottom_neighbour(index) {
            is_lowest &= current_height < bottom.height;
        }
        is_lowest
    }

    fn risk_level(&self, index: usize) -> usize {
        if self.is_lowest_point(index) {
            println!("lowest point is at index {}", index);
            return &self.locations[index].height + &1;
        }
        return 0;
    }

    fn risk_level_sum(&self) -> usize {
        self.locations.iter().enumerate().map(|(i,_)| self.risk_level(i)).sum()
    }
}

impl Location {
    fn new(height: usize) -> Location {
        Location {
            height: height
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

    let mut height_map = HeightMap::init();
    height_map.read_input(&mut lines);

    println!("Risk level: {}", height_map.risk_level_sum());
    
}