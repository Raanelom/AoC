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

fn generate_builders() -> Vec<RegexBuilder> {
    let opening_brackets = ['(', '[', '{', '<'];
    let closing_brackets = [')', ']', '}', '>'];
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

fn scan_callback<'t>(n: i32, caps: Captures<'t>) -> bool {
    println!("scan: {}", n);
    println!("match at {}", caps.offset());

    for (i, cap) in caps.iter_pos().enumerate() {
        match cap {
            Some(pos) => println!("{}: {:?}", i, pos),
            None => println!("{}: did not capture", i),
        }
    }

    true
}

fn determine_error_score(regex_builders: Vec<RegexBuilder>) {
    for regex_builder in regex_builders {
        let string = "(({([{}])<><>[]}))";
        let test_string = "[({(<(())[]>[[{[]{<()<>>";
        let faulty_string = "[{[{({}]{}}([{[{{{}}([]";
        regex_builder.generate_regex().scan(test_string, scan_callback)
            // Some(caps) => {
            //     // We have at least one match
            //     println!("match at {}", caps.offset());
            //     for (i, cap) in caps.iter_pos().enumerate() {
            //         match cap {
            //             Some(pos) => println!("{}: {:?}", i, pos),
            //             None => println!("{}: did not capture", i),
            //         }
            //     }
            // }
            // None => println!("search fail"),
        ;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);

    let regex_builders = generate_builders();

    determine_error_score(regex_builders);
        
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

