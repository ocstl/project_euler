use std::fs;

const INPUT: &str = "inputs/p022_names.txt";

fn alphabetical_value(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .map(|c| (c + 1 - b'A') as u64)
        .sum()
}

fn main () {
    let mut names: Vec<String> = fs::read_to_string(INPUT)
        .expect("Missing file.")
        .split(',')
        .map(|name| name.trim_matches('"').to_string())
        .collect();

    names.sort();

    let answer: u64 = names.iter()
        .enumerate()
        .map(|(idx, name)| (idx as u64 + 1) * alphabetical_value(name))
        .sum();

    println!("Answer: {}", answer);
}