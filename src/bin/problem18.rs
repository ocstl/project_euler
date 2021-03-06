use std::fs;

const INPUT_FILE: &str = "inputs/problem18.txt";

/// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
/// the maximum total from top to bottom is 23.
/// Find the maximum total from top to bottom of the triangle below <problem18.txt>.
fn main() -> Result<(), std::io::Error> {
    let mut input = fs::read_to_string(INPUT_FILE)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    // Start with the last line, picking the maximum value to combine with the preceding line.
    let initial = input.pop().unwrap();
    let answer = input.iter().rev().fold(initial, |current, line| {
        current
            .windows(2)
            .zip(line.iter())
            .map(|(window, p)| window.iter().max().unwrap() + p)
            .collect()
    })[0];

    println!("The answer is: {}", answer);
    Ok(())
}
