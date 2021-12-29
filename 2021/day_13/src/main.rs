use std::env;
use util::input_operations::{StringOperations};
use std::collections::HashSet;
use ndarray::{Array2,prelude::*};
use itertools::Itertools;

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Manual {
    dot_sheet: Array2<usize>,
    instructions: Vec<(char,usize)>
}

impl Manual {
    fn read(dot_sheet_raw: &Vec<&str>, instructions_raw: &Vec<&str>) -> Manual {
        let dot_sheet_parsed = dot_sheet_raw
            .iter()
            .map(|coord| coord.split(",")
                .map(|point| point.parse().unwrap())
                .collect_tuple().unwrap())
                .collect::<Vec<(usize, usize)>>();
        // Get dimensions
        let x_max = dot_sheet_parsed.iter().map(|coord| coord.0).max().unwrap() + 1;
        let y_max = dot_sheet_parsed.iter().map(|coord| coord.1).max().unwrap() + 1;
        // Initialize 2D-array
        let mut dot_sheet = Array2::<usize>::from_elem((y_max, x_max), 1);
        println!("Max dimensions: x={}, y={}", x_max, y_max);
        dot_sheet_parsed.iter().for_each(|coord| {
            // 0 represents a "#", as it makes matrix multiplication easier
            dot_sheet[[coord.1, coord.0]] = 0;
        });
        let instructions_parsed = instructions_raw
            .iter()
            .map(|x| x.split(" ").collect::<Vec<_>>()[2].split("=").collect::<Vec<_>>())
            .map(|x| (x[0].parse::<char>().unwrap(),x[1].parse::<usize>().unwrap()))
            .rev()
            .collect::<Vec<(char,usize)>>();

        Manual {
            dot_sheet: dot_sheet,
            instructions: instructions_parsed
        }
    }

    fn fold_all(&mut self) {
        while self.fold_next() {
            println!("Still folding");
        }
    }
    
    fn fold_next(&mut self) -> bool {
        if let Some(next_instruction) = self.instructions.pop() {
            if next_instruction.0 == 'y' {
                let upper_part = self.dot_sheet.slice(s![0..next_instruction.1, ..]);
                let lower_part_reverse = self.dot_sheet.slice(s![(1+next_instruction.1)..self.dot_sheet.shape()[0];-1, ..]);
                let combination = &upper_part*&lower_part_reverse;
                self.dot_sheet = combination;
            }
            else if next_instruction.0 == 'x' {
                let left_part = self.dot_sheet.slice(s![.., 0..next_instruction.1]);
                let right_part_reverse = self.dot_sheet.slice(s![.., (1+next_instruction.1)..self.dot_sheet.shape()[1];-1]);
                let combination = &left_part*&right_part_reverse;
                self.dot_sheet = combination;
            }
            else {
                panic!("Invalid instruction was given. It should be either on the 'x' or 'y'-axis")
            }
            
            return true;
        }
        else {
            println!("There's nothing left to fold");
            return false;
        }
    }

    fn count_dots(&self) -> usize{
        self.dot_sheet.iter().filter(|x| *x == &0).count()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = args[1].read_file_to_string();
    let input = input.split_double_newlines()
        .map(|x| x.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut manual = Manual::read(&input[0], &input[1]);
    manual.fold_all();
    println!("Number of dots: {}", manual.count_dots());
    println!("{:#?}", manual.dot_sheet);
}