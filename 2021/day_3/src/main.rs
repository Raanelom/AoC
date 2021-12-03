use std::env;
use util::input_operations::{read_file_to_string,split_lines};

struct DiagnosticReport {
    items: Vec<usize>
}

impl DiagnosticReport {
    fn values(&self) -> std::iter::Peekable<std::slice::Iter<'_, usize>> {
        self.items.iter().peekable()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a filename as argument");
    }
    let input = read_file_to_string(&args[1]);
    let entry_size = split_lines(&input).next().map(|x| x.len()).unwrap();
    let diagnostic_report = to_report(input);
    reduce(diagnostic_report, entry_size);
}

fn to_report(diagnostic_report: std::string::String) -> DiagnosticReport {
    DiagnosticReport {
        items: split_lines(&diagnostic_report)
            .map(|entry| usize::from_str_radix(entry, 2).unwrap()).collect()
    }
}

fn reduce(diagnostic_report: DiagnosticReport, entry_size: usize) {
    let report_entry_count: usize = diagnostic_report.values().count();
    let threshold: usize = (report_entry_count / 2) + (report_entry_count % 2);
    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;
    for n in 0..entry_size {
        let mut no_of_ones: usize = 0;
        let mut report_entries = diagnostic_report.values();
        while report_entries.peek().is_some() {
            let x = report_entries.next().unwrap();
            no_of_ones += (x >> n) & 1;
            
        }
        println!("Found {} ones at position {}", no_of_ones, n);
        gamma_rate = gamma_rate + if no_of_ones > threshold { 1<<n } else { 0 };
        epsilon_rate = epsilon_rate + if no_of_ones < threshold { 1<<n } else { 0 };
        println!("result is now {}", format!("{:#6b}", gamma_rate));
        println!("result is now {}", format!("{:#6b}", epsilon_rate));
    }
    println!("gamma rate: {}", gamma_rate);
    println!("{}", format!("{:#6b}", gamma_rate));

    println!("epsilon rate: {}", epsilon_rate);
    println!("{}", format!("{:#6b}", epsilon_rate));

    println!("The answer is {}", epsilon_rate*gamma_rate);
    
}