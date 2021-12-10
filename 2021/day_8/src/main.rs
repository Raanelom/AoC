use std::env;
use util::input_operations::{read_file_to_string,split_lines};

#[derive(Debug)]
struct Wire {
    pattern: Vec<char>
}

impl Wire {
    fn from(pattern: Vec<char>) -> Wire {
        let mut sorted_pattern = pattern;
        sorted_pattern.sort();
        Wire {
            pattern: sorted_pattern
        }
    }
}

#[derive(Debug)]
struct Segment {
    pattern: Vec<char>,
    length: usize,
    value: Option<usize>
}

impl Segment {
    fn from(pattern: Vec<char>) -> Segment {
        let length = pattern.len();
        let mut sorted_pattern = pattern;
        sorted_pattern.sort();
        Segment {
            pattern: sorted_pattern,
            length: length,
            value: match length {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None
            }
        }
    }
}

#[derive(Debug)]
struct Display {
    wires: Vec<Wire>,
    segments: Vec<Segment>
}

impl Display {
    fn from_input(line: Vec<&str>) -> Display {
        assert_eq!(2, line.len());
        Display {
            wires: line[0].split(" ").map(|x| Wire::from(x.chars().collect())).collect(),
            segments: line[1].split(" ").map(|x| Segment::from(x.chars().collect())).collect()
        }
    }

    fn count_unique_segments(&self) -> usize {
        let mut count: usize = 0;
        for segment in self.segments.iter() {
            if let Some(_x) = segment.value {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let lines = split_lines(&input);
    let displays: Vec<Display> = lines
        .map(|x| Display::from_input(x.split(" | ").collect())).collect();
    println!("{:?}", displays[0].wires[0]);
    println!("{:?}", displays[0].wires[1]);

    let unique_segment_count: usize = displays.iter().map(|x| x.count_unique_segments()).sum();
    println!("{}", unique_segment_count);
}

// 1 = 2 segments => unique
// 2 = 5 segments
// 3 = 5 segments
// 4 = 4 segments => unique
// 5 = 5 segments
// 6 = 6 segments
// 7 = 3 segments => unique
// 8 = 7 segments => unique
// 9 = 6 segments

// 1 and 6 have in common: 1 line
// 1 and 9 have in common: 2 lines

// 6 and 9 have in common: 5 lines

// . . .  . . .
// .      .   .
// . . .  . . .
// .   .      .
// . . .  . . .
