use project_euler::roman::Roman;
use std::convert::TryFrom;
use std::fs;

const FILENAME: &str = "inputs/p089_roman.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Missing file.");
    let to_minimal_form = |line: &str| -> Result<String, &'static str> {
        Ok(Roman::from(u32::try_from(&Roman::new(line))?).to_string())
    };

    let answer: usize = input
        .lines()
        .map(|line| line.len() - to_minimal_form(line).unwrap().len())
        .sum();

    println!("The answer is: {}", answer);
}
