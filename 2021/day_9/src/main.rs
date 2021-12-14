use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_chars};
use std::collections::HashSet;

#[derive(Debug,Clone)]
struct HeightMap {
    map_width: usize,
    map_height: usize,
    locations: Vec<Location>,
    basins: Vec<Basin>
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Location {
    index: usize,
    height: usize
}

#[derive(Debug,Clone)]
struct Basin {
    locations: HashSet<Location>
}

impl HeightMap {
    fn init() -> HeightMap {
        HeightMap {
            map_width: 0,
            map_height: 0,
            locations: Vec::new(),
            basins: Vec::new()
        }
    }

    fn read_input(&mut self, lines: &mut std::iter::Peekable<std::str::Lines>) {
        if let Some(line) = lines.peek() {
            self.map_width = line.len();
        }
        for line in lines {
            let mut new_locations: Vec<Location> = split_chars(&line.to_string())
                .filter(|x| x.is_digit(10))
                .enumerate()
                .map(|(i,x)| Location::new(x.to_digit(10).unwrap() as usize, i + (self.map_height*self.map_width)))
                .collect::<Vec<Location>>();
            self.locations.append(&mut new_locations);
            self.map_height += 1;
        }
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

    fn neighbours(&self, index: usize) -> Vec<usize> {
        let mut neighbour_indices: Vec<usize> = Vec::new();
        if let Some(left) = self.left_neighbour(index) {
            neighbour_indices.push(left.index);
        }
        if let Some(top) = self.top_neighbour(index) {
            neighbour_indices.push(top.index);
        }
        if let Some(right) = self.right_neighbour(index) {
            neighbour_indices.push(right.index);
        }
        if let Some(bottom) = self.bottom_neighbour(index) {
            neighbour_indices.push(bottom.index);
        }
        neighbour_indices
    }

    fn is_lowest_point(&self, index: usize) -> bool {
        let current_height = self.locations[index].height;
        let mut is_lowest = true;
        for neighbour_idx in self.neighbours(index).iter() {
            is_lowest &= current_height < self.locations[*neighbour_idx].height;
        }
        is_lowest
    }

    fn lowest_points(&self) -> Vec<usize> {
        self.locations.iter()
            .filter(|x| self.is_lowest_point(x.index))
            .map(|x| x.index)
            .collect()
    }

    fn risk_level(&self, index: usize) -> usize {
        return &self.locations[index].height + &1;
    }

    fn risk_level_sum(&self) -> usize {
        self.lowest_points()
            .iter()
            .map(|i| self.risk_level(*i))
            .sum()
    }

    fn find_bassins(&mut self) {
        self.basins.retain(|_x| true); // Remove all elements
        for lowest_point in self.lowest_points().into_iter() {
            let mut basin = Basin::init();
            self.fill_basin(lowest_point, &mut basin);
            self.basins.push(basin);
        }
    }

    fn find_largest_basins(&mut self) -> usize {
        self.find_bassins();
        self.basins
            .sort_by(|a,b| b.size().cmp(&a.size()));
        let basin_length = self.basins.iter().len();
        let largest_basins = self.basins[0..std::cmp::min(3,basin_length)].iter();
        let mut basin_multiplication = 1;
        largest_basins.for_each(|x| basin_multiplication *= x.size());
        return basin_multiplication
    }

    fn fill_basin(&self, index: usize, basin: &mut Basin) {
        let current_location = &self.locations[index];
        if !basin.locations.contains(current_location) && current_location.height < 9 {
            basin.locations.insert(current_location.clone());
            for neighbour_index in self.neighbours(index) {
                self.fill_basin(neighbour_index, basin);
            }
        }
    }
}

impl Location {
    fn new(height: usize, index: usize) -> Location {
        Location {
            height: height,
            index: index
        }
    }
}

impl Basin {
    fn init() -> Basin {
        Basin {
            locations: HashSet::new()
        }
    }

    fn size(&self) -> usize {
        self.locations.iter().count()
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

    println!("Basins: {:?}", height_map.find_largest_basins());
    
}