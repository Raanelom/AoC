use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_chars};
use std::collections::HashSet;
use onig::*;
use phf::phf_map;

// https://github.com/kkos/oniguruma/blob/master/doc/RE
// https://docs.rs/phf/latest/phf/
// https://github.com/rust-onig/rust-onig/blob/main/onig/examples/scan.rs

static SYNTAX_ERROR_SCORE_MAP: phf::Map<char, usize> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
};

static PARENTHESES_REWARD_MAP: phf::Map<char, usize> = phf_map! {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4
};


const REGEX_GROUP_NAMES: [&str; 5] = ["a", "b", "c", "d", "e"];


#[derive(Debug,Clone)]
struct RegexBuilder {
    regex_chunks: Vec<RegexChunk>,
    syntax_error_score: usize
}


#[derive(Debug,Clone)]
struct RegexChunk {
    opening_bracket: char,
    closing_bracket: char,
}

impl RegexBuilder {
    fn new(syntax_error_score: usize, regex_chunks: Vec<RegexChunk>) -> RegexBuilder {
        RegexBuilder {
            regex_chunks: regex_chunks,
            syntax_error_score: syntax_error_score
        }
    }

    fn concat_children(&self) -> String {
        let mut group_names = REGEX_GROUP_NAMES.iter();
        self.regex_chunks.iter()
            .map(|x| x.generate_regex(group_names.next().unwrap()))
            .collect::<Vec<String>>()
            .join("|")
    }

    fn generate_regex_string(&self) -> String {
        [r"(?<re>", &self.concat_children(), ")+"].join("")
    }
    
    fn generate_regex(&self) -> Regex {
        Regex::new(&self.generate_regex_string()).unwrap()
    }
}

impl RegexChunk {
    fn new(opening_bracket: char, closing_bracket: char) -> RegexChunk {
        RegexChunk {
            opening_bracket: opening_bracket, 
            closing_bracket: closing_bracket
        }
    }
    fn generate_regex(&self, groupname: &str) -> String {
        // Add an extra escape character
        let opening_bracket = 
            if self.opening_bracket == '[' { String::from(r"\[") } 
            else { self.opening_bracket.to_string() };
        let closing_bracket = 
            if self.closing_bracket == ']' { String::from(r"\]") } 
            else { self.closing_bracket.to_string() };
        // Generate the actual regex
        return [r"(?<", groupname, ">", &format!(r"\{}", self.opening_bracket), 
            r"(?:(?> [^", &opening_bracket, &closing_bracket, r"]+ )|\g<re>)*", 
            &format!(r"\{}", self.closing_bracket), r")"].join("");
    }
}

fn generate_builders(opening_brackets: [char; 4], closing_brackets: [char; 4]) -> Vec<RegexBuilder> {
    // Determine basis-regex
    let mut regex_builders = Vec::<RegexBuilder>::new();
    let mut regex_chunks: Vec<RegexChunk> = Vec::new();
    for i in 0..opening_brackets.iter().len() {
        regex_chunks.push(RegexChunk::new(opening_brackets[i], closing_brackets[i]))
    }
    let regex_chunks_base = regex_chunks.clone();
    regex_builders.push(RegexBuilder::new(0, regex_chunks));

    // And now add some additional builders
    let size = opening_brackets.len();
    for offset in 1..size {
        for i in 0..opening_brackets.iter().len() {
            let mut regex_chunks = regex_chunks_base.clone();
            let closing_bracket = closing_brackets[(i+offset)%size];
            regex_chunks.push(RegexChunk::new(opening_brackets[i], closing_bracket));
            // Add generated regex (base-regex + error-detection-regex) and determine syntax error score
            regex_builders.push(RegexBuilder::new(SYNTAX_ERROR_SCORE_MAP[&closing_bracket], regex_chunks))
        }
    }
    return regex_builders;
}

fn determine_error_score(regex_builders: &Vec<RegexBuilder>, ref_regex: &Regex, input: &str) -> usize {
    for regex_builder in regex_builders {
        let mut closing_bracket_ranges: HashSet<usize> = HashSet::new();
        for caps in regex_builder.generate_regex().captures_iter(input) {
            // We have at least one match
            for capture in caps.iter_pos() {
                if let Some(pos_range) = capture {
                    for pos in pos_range.0..pos_range.1 {
                        closing_bracket_ranges.insert(pos);
                    }
                }
            }
        }
        let mut is_valid = true;
        for caps in ref_regex.captures_iter(input) {
            // We have at least one match
            for (i, capture) in caps.iter_pos().enumerate() {
                if let Some(pos_range) = capture {
                    is_valid &= closing_bracket_ranges.contains(&pos_range.0);
                }
            }
        }
        if is_valid {
            return regex_builder.syntax_error_score
        }        
    }
    panic!("Expected an early return");
}

fn determine_autocompletion_reward(valid_lines: Vec<&str>, regex_builder: &RegexBuilder) -> usize {
    let mut scores = Vec::new();
    for line in valid_lines.iter() {
        let mut score = 0;
        for bracket_reward in regex_builder.generate_regex()
            .replace_all(line, "")
            .chars()
            .map(|x| PARENTHESES_REWARD_MAP[&x])
            .rev() {
            score = (score * 5) + bracket_reward;
        }
        scores.push(score);
    }
    scores.sort();
    return scores[scores.len()/2];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let lines = split_lines(&input);

    let opening_brackets = ['(', '[', '{', '<'];
    let closing_brackets = [')', ']', '}', '>'];
    let ref_regex_str = ["(", &closing_brackets
        .iter()
        .map(|x| [r"(\", &x.to_string(), ")"].join(""))
        .collect::<Vec<String>>()
        .join("|"), ")"].join("");
    let ref_regex = Regex::new(&ref_regex_str).unwrap();
    let regex_builders = generate_builders(opening_brackets, closing_brackets);
    let mut total_score = 0;
    let mut valid_lines = Vec::new();
    for line in lines {
        let score = determine_error_score(&regex_builders, &ref_regex, line);
        if score == 0 {
            valid_lines.push(line);
        }
        total_score += determine_error_score(&regex_builders, &ref_regex, line);
    }
    println!("Score: {}", total_score);

    let reward = determine_autocompletion_reward(valid_lines, &regex_builders[0]);

    println!("Reward: {}", reward);
}
