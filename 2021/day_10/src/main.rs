use std::env;
use util::input_operations::{read_file_to_string,split_lines,split_chars};
use std::collections::HashSet;
use onig::*;
use phf::phf_map;

static SYNTAX_ERROR_SCORE_MAP: phf::Map<char, usize> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
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
        self.regex_chunks.iter().map(|x| x.generate_regex(group_names.next().unwrap())).collect::<Vec<String>>().join("|")
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
        let opening_bracket = 
            if self.opening_bracket == '[' { String::from(r"\[") } 
            else { self.opening_bracket.to_string() };
        let closing_bracket = 
            if self.closing_bracket == ']' { String::from(r"\]") } 
            else { self.closing_bracket.to_string() };
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
            regex_builders.push(RegexBuilder::new(SYNTAX_ERROR_SCORE_MAP[&closing_bracket], regex_chunks))
        }
    }
    return regex_builders;
}

fn determine_error_score(regex_builders: Vec<RegexBuilder>, ref_regex: Regex, input: &str) -> usize {
    for regex_builder in regex_builders {
        let mut closing_bracket_ranges: HashSet<usize> = HashSet::new();
        for caps in regex_builder.generate_regex().captures_iter(input) {
            // We have at least one match
            println!("match at {}", caps.offset());
            for (i, capture) in caps.iter_pos().enumerate() {
                match capture {
                    Some(pos_range) => {
                        println!("{}: {:?}", i, pos_range);
                        for pos in pos_range.0..pos_range.1 {
                            closing_bracket_ranges.insert(pos);
                        }
                    },
                    None => println!("{}: did not capture", i),
                }
            }
        }
        let mut is_valid = true;
        for caps in ref_regex.captures_iter(input) {
            // We have at least one match
            println!("match at {}", caps.offset());
            for (i, capture) in caps.iter_pos().enumerate() {
                match capture {
                    Some(pos_range) => {
                        println!("{}: {:?}", i, pos_range);
                        is_valid &= closing_bracket_ranges.contains(&pos_range.0);
                    },
                    None => println!("{}: did not capture", i),
                }
            }
        }
        println!("Is valid: {:?}", is_valid);
        if is_valid {
            return regex_builder.syntax_error_score
        }        
    }
    panic!("Expected an early return");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);

    let opening_brackets = ['(', '[', '{', '<'];
    let closing_brackets = [')', ']', '}', '>'];
    let ref_regex_str = ["(", &closing_brackets
        .iter()
        .map(|x| [r"(\", &x.to_string(), ")"].join(""))
        .collect::<Vec<String>>()
        .join("|"), ")"].join("");
    let ref_regex = Regex::new(&ref_regex_str).unwrap();
    let regex_builders = generate_builders(opening_brackets, closing_brackets);

    let string = "(({([{}])<><>[]}))";
    let test_string = "[({(<(())[]>[[{[]{<()<>>";
    let faulty_string = "[{[{({}]{}}([{[{{{}}([]";
    let score = determine_error_score(regex_builders, ref_regex, faulty_string);
    println!("Score: {}", score);
        
    // for regex_builder in regex_builders {
    //     let string = "(({([{}])<><>[]}))";
    //     let faulty_string = "[{[{({}]{}}([{[{{{}}([]";
    //     match regex_builder.generate_regex().captures(string) {
    //         Some(caps) => {
    //             println!("match at {}", caps.offset());
    //             for (i, cap) in caps.iter_pos().enumerate() {
    //                 match cap {
    //                     Some(pos) => println!("{}: {:?}", i, pos),
    //                     None => println!("{}: did not capture", i),
    //                 }
    //             }
    //         }
    //         None => println!("search fail"),
    //     }

    // }
    // let incorrect_parentheses_a = r"(?<e>\((?:(?> [^(\]]+ )|\g<re>)*\])";
    // let correct_expression = [r"(?<re>", &correct_parentheses_a, "|", correct_parentheses_b, "|", correct_parentheses_c, "|", correct_parentheses_d, "|", incorrect_parentheses_a, ")"].join("");
    // let correct_expression = regex_builders[0].generate_regex();
    // let regex = Regex::new(&correct_expression).unwrap();
    // let string = "(({([{}])<><>[]}))";
    // let faulty_string = "[{[{({}]{}}([{[{{{}}([]";
    // // [<>({}){}[([])<>]]
    // match regex.captures(string) {
    //     Some(caps) => {
    //         println!("match at {}", caps.offset());
    //         for (i, cap) in caps.iter_pos().enumerate() {
    //             match cap {
    //                 Some(pos) => println!("{}: {:?}", i, pos),
    //                 None => println!("{}: did not capture", i),
    //             }
    //         }
    //     }
    //     None => println!("search fail"),
    // }

}

