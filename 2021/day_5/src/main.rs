use std::env;
use util::input_operations::{read_file_to_string,split_lines};
use std::collections::HashMap;

#[derive(Debug)]
struct Direction {
    x_positive: bool,
    y_positive: bool
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal(Direction)
}

#[derive(Debug)]
struct LineSegment {
    start: (usize, usize),
    end: (usize, usize),
    orientation: Orientation
}

impl LineSegment {
    fn new(line: Vec<Vec<usize>>) -> LineSegment {
        assert_eq!(line.iter().count(), 2);
        for coords in line.iter() {
            assert_eq!(coords.iter().count(), 2);
        }
        let start_coord = (line[0][0], line[0][1]);
        let end_coord = (line[1][0], line[1][1]);
        LineSegment {
            start: start_coord,
            end: end_coord,
            orientation: if start_coord.0 == end_coord.0 { Orientation::Vertical } 
                else if start_coord.1 == end_coord.1 { Orientation::Horizontal } 
                else { 
                    Orientation::Diagonal(Direction {
                        x_positive: start_coord.0 < end_coord.0,
                        y_positive: start_coord.1 < end_coord.1,
                    }) 
                }
        }
    }

    fn line_coordinates(&self) -> Vec<(usize, usize)> {
        let mut in_between = Vec::new();
        match &self.orientation {
            Orientation::Horizontal => {
                // TODO: worry about the direction
                for x in std::cmp::min(self.start.0, self.end.0)..=std::cmp::max(self.start.0, self.end.0) {
                    in_between.push((x, self.start.1));
                }
            }
            Orientation::Vertical => {
                // TODO: worry about the direction
                for y in std::cmp::min(self.start.1, self.end.1)..=std::cmp::max(self.start.1, self.end.1) {
                    in_between.push((self.start.0, y));
                }
            }
            Orientation::Diagonal(direction) => {
                let min_y = std::cmp::min(self.start.1, self.end.1);
                let number_of_points = std::cmp::max(self.start.1, self.end.1) - min_y;
                if (direction.x_positive && direction.y_positive) 
                    || (!direction.x_positive && !direction.y_positive) {
                    let min_x = std::cmp::min(self.start.0, self.end.0);
                    for i in 0..=number_of_points {
                        in_between.push((min_x+i, min_y+i));
                    }
                }
                else {
                    let max_x = std::cmp::max(self.start.0, self.end.0);
                    for i in 0..=number_of_points {
                        in_between.push((max_x-i, min_y+i));
                    }
                }
            }
        }
        in_between
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let lines = split_lines(&input);
    let line_segments = lines.map(|x| LineSegment::new(
            x.split(" -> ")
            .map(|y| y.split(",")
                .map(|z| z.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>()))
        .collect::<Vec<LineSegment>>();

    determine_line_collision(line_segments);
}

fn determine_line_collisions(line_diagram: HashMap<(usize, usize), usize>) {
    let res: usize = line_diagram.values().filter(|coord| coord > &&1).count();
    println!("{}", res);
}

fn determine_line_collision(line_segments: std::vec::Vec<LineSegment>) {
    let mut line_diagram: HashMap<(usize, usize), usize> = HashMap::new();

    for line_segment in line_segments {
        for point in line_segment.line_coordinates() {
            // Insert returns the old value. If line_diagram already contains a value, increase it with one
            if let Some(val) = line_diagram.insert(point, 1) {
                line_diagram.insert(point, val + 1);
            };
        }
    }
    determine_line_collisions(line_diagram);

}