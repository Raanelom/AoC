
pub mod input_operations {
    use std::fs;

    pub fn read_file_to_string(filename: &std::string::String) -> String {
        println!("Read file '{}'", filename);
    
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file")
    }

    pub fn split_lines(input: &String) -> std::iter::Peekable<std::str::Lines>
    {
        input.lines().peekable()
    }

    pub fn split_double_newlines(input: &String) -> std::iter::Peekable<std::str::Split<&str>>
    {
        input.split("\n\n").peekable()
    }

    pub fn split_whitespace(input: &String) -> std::iter::Peekable<std::str::SplitWhitespace>
    {
        input.split_whitespace().peekable()
    }
}