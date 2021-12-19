use std::env;
use util::input_operations::{read_file_to_string,split_lines};
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug,Eq,Hash,Clone)]
struct Segment {
    pattern: Vec<char>,
    length: usize,
    value: Option<usize>
}


impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        if let Some(x) = self.value {
            if let Some(y) = other.value {
                return x == y;
            }
        }
        false
    }
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

    fn to_hashset(&self) -> HashSet<char> {
        HashSet::from_iter(self.pattern.iter().map(|x| x.clone()))
    }
}

#[derive(Debug,Clone)]
struct Display {
    signal_patterns: Vec<Segment>,
    output: Vec<Segment>
}

impl Display {
    fn from_input(line: Vec<&str>) -> Display {
        assert_eq!(2, line.len());
        Display {
            signal_patterns: line[0]
                .split(" ")
                .map(|x| Segment::from(x.chars().collect()))
                .collect(),
            output: line[1]
                .split(" ")
                .map(|x| Segment::from(x.chars().collect()))
                .collect()
        }
    }

    fn count_unique_output_patterns(&self) -> usize {
        let mut count: usize = 0;
        for segment in self.output.iter() {
            if let Some(_x) = segment.value {
                count += 1;
            }
        }
        count
    }

    fn unique_patterns(&self) -> HashSet<Segment> {
        let mut unique_patterns: HashSet<Segment> = HashSet::from_iter(self.signal_patterns
                .iter()
                .filter(|x| x.value.is_some())
                .map(|x| x.clone()));
        for signal_pattern in self.output.iter().filter(|x| x.value.is_some()) {
            unique_patterns.insert(signal_pattern.clone());
        }
        unique_patterns
    }

    fn determine_signal_pattern_values(&self, signal_patterns: &mut Vec<Segment>) -> bool {
        let mut is_determined = true;
        let unique_patterns = self.unique_patterns();
        for pattern in signal_patterns.iter_mut().filter(|m| m.value == None) {
            assert!([5,6].contains(&pattern.length));
            for unique_pattern in unique_patterns.iter() {
                let unique_pattern_hash = unique_pattern.to_hashset();
                let current_pattern_hash = pattern.to_hashset();
                let intersection: usize = unique_pattern_hash.intersection(&current_pattern_hash).collect::<HashSet<_>>().len();
                match unique_pattern.value {
                    Some(1) => {
                        // 1 and 6 have in common: 1 line
                        if intersection == 1 && pattern.length == 6  {
                            pattern.value = Some(6);
                        }
                        // 1 and 3 have in common: 2 lines => unique
                        else if intersection == 2 && pattern.length == 5 {
                            pattern.value = Some(3);
                        }
                    }
                    Some(4) => {
                        // 4 and 6 have in common: 3 lines
                        if intersection == 4 && pattern.length == 6  {
                            pattern.value = Some(9);
                        }
                        // 4 and 2 have in common: 2 lines => unique
                        else if intersection == 2 && pattern.length == 5  {
                            pattern.value = Some(2);
                        }
                    }
                    Some(5) => {
                        // 5 and 0 have in common: 4 lines
                        if intersection == 4 && pattern.length == 6 {
                            pattern.value = Some(0);
                        }
                    }
                    Some(6) => {
                        // 6 and 5 have in common: 5 lines => unique
                        if intersection == 5 && pattern.length == 5  {
                            pattern.value = Some(5);
                        }
                    }
                    Some(7) => {
                        // 7 and 6 have in common: 2 lines
                        if intersection == 2 && pattern.length == 6  {
                            pattern.value = Some(6);
                        }
                        // 7 and 3 have in common: 3 lines => unique
                        else if intersection == 3 && pattern.length == 5 {
                            pattern.value = Some(3);
                        }
                    }
                    Some(9) => {
                        // 9 and 2 have in common: 4 lines => unique
                        if intersection == 4 && pattern.length == 5 {
                            pattern.value = Some(2);
                        }
                    }
                    _ => {
                        is_determined = false;
                    }
                }
            }
        }
        is_determined
    }
    

    fn determine_output_values(&self) -> usize {
        self.output.iter()
            .map(|x| x.value.unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let lines = split_lines(&input);
    let mut displays: Vec<Display> = lines
        .map(|x| Display::from_input(x.split(" | ").collect())).collect();

    let unique_segment_count: usize = displays.iter().map(|x| x.count_unique_output_patterns()).sum();
    println!("{}", unique_segment_count);

    let mut total_output = 0;
    for display in displays.iter_mut() {
        let mut is_determined = false;
        while !is_determined {
            let mut signal_patterns = display.signal_patterns.clone();
            is_determined = display.determine_signal_pattern_values(&mut signal_patterns);
            display.signal_patterns = signal_patterns;
        }
        is_determined = false;
        while !is_determined {
            let mut output = display.output.clone();
            is_determined = display.determine_signal_pattern_values(&mut output);
            display.output = output;
        }
        let output = display.determine_output_values();
        total_output += output;
        println!("done. Output is {}", output);
    }
    println!("Total output is {}", total_output);
}


////////////////
// Draft Area //
////////////////
// 1 = 2 segments => unique
// 2 = 5 segments
// 3 = 5 segments
// 4 = 4 segments => unique
// 5 = 5 segments
// 6 = 6 segments
// 7 = 3 segments => unique
// 8 = 7 segments => unique
// 9 = 6 segments

// 1 and 0 have in common: 2 lines
// 1 and 6 have in common: 1 line => unique
// 1 and 9 have in common: 2 lines
// 4 and 0 have in common: 3 lines
// 4 and 6 have in common: 3 lines
// 4 and 9 have in common: 4 lines => unique
// 7 and 0 have in common: 3 lines
// 7 and 6 have in common: 2 lines => unique
// 7 and 9 have in common: 3 lines
// 8 and 0 have in common: 6 lines
// 8 and 6 have in common: 6 lines
// 8 and 9 have in common: 6 lines

// 1 and 2 have in common: 1 line
// 1 and 3 have in common: 2 lines => unique
// 1 and 5 have in common: 1 line
// 4 and 2 have in common: 2 lines => unique
// 4 and 3 have in common: 3 lines
// 4 and 5 have in common: 3 lines
// 6 and 2 have in common: 4 lines
// 6 and 3 have in common: 4 lines
// 6 and 5 have in common: 5 lines => unique
// 7 and 2 have in common: 2 lines
// 7 and 3 have in common: 3 lines => unique
// 7 and 5 have in common: 2 lines
// 9 and 2 have in common: 4 lines => unique
// 9 and 3 have in common: 5 lines
// 9 and 5 have in common: 5 lines

// . . .  . . .
// .      .   .
// . . .  . . .
// .   .      .
// . . .  . . .
