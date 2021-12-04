use std::env;
use util::input_operations::{read_file_to_string,split_lines};

#[derive(Clone)]
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
    // Determine the size of a single line (number of bits)
    let entry_size = split_lines(&input).next().map(|x| x.len()).unwrap();
    let diagnostic_report = to_report(input);
    let (epsilon_rate, gamma_rate) = calculate_power_consumption(diagnostic_report.clone(), entry_size);
    println!("The answer is {}", epsilon_rate*gamma_rate);

    calculate_life_support_rating(diagnostic_report, entry_size);
}

fn to_report(diagnostic_report: std::string::String) -> DiagnosticReport {
    DiagnosticReport {
        items: split_lines(&diagnostic_report)
            .map(|entry| usize::from_str_radix(entry, 2).unwrap()).collect()
    }
}

fn calculate_power_consumption(diagnostic_report: DiagnosticReport, entry_size: usize) -> (usize, usize) {
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
        gamma_rate = gamma_rate + if no_of_ones >= threshold { 1<<n } else { 0 };
        epsilon_rate = epsilon_rate + if no_of_ones <= threshold { 1<<n } else { 0 };
    }
    (gamma_rate, epsilon_rate)    
}

fn reduce(report_entries: Vec<usize>, bit_shift: usize, return_largest_set: bool) -> Vec<usize> {
    let ones: Vec<usize> = report_entries.clone().into_iter().filter(|x| ((x >> bit_shift) & 1) == 1).collect();
    let zeros: Vec<usize> = report_entries.into_iter().filter(|x| ((x >> bit_shift) & 1) == 0).collect();
    let ones_count: usize = ones.iter().count();
    let zeros_count: usize = zeros.iter().count();
    return match return_largest_set {
        false => if zeros_count <= ones_count { zeros } else { ones }
        true => if ones_count >= zeros_count { ones } else { zeros }
    }
}


fn calculate_life_support_rating(diagnostic_report: DiagnosticReport, entry_size: usize) {
    let mut oxygen_report_entries: Vec<usize> = diagnostic_report.items.clone().into_iter().collect();
    let mut co2_scrubber_report_entries: Vec<usize> = diagnostic_report.items.clone().into_iter().collect();
    for n in (0..entry_size).rev() {
        if oxygen_report_entries.iter().count() > 1 {
            oxygen_report_entries = reduce(oxygen_report_entries, n, true);
        }
        if co2_scrubber_report_entries.iter().count() > 1 {
            co2_scrubber_report_entries = reduce(co2_scrubber_report_entries, n, false);
        }

        
        
    }
    assert_eq!(oxygen_report_entries.iter().count(), 1);
    assert_eq!(co2_scrubber_report_entries.iter().count(), 1);

    let oxygen_generator_rating: &usize = oxygen_report_entries.iter().next().unwrap();
    let co2_scrubber_rating: &usize = co2_scrubber_report_entries.iter().next().unwrap();
    println!("oxygen_generator_rating: {}", oxygen_generator_rating);
    println!("co2_scrubber_rating: {}", co2_scrubber_rating);
    println!("Life time support rating is {}", oxygen_generator_rating*co2_scrubber_rating);
}