use std::fs;

fn main() {
    let mut input: Vec<Vec<usize>> =
        fs::read_to_string("inputs/problem18.txt").unwrap()
            .lines()
            .map(|line|
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>())
            .collect();

    let init = input.pop().unwrap();

    let answer = input.iter().rev()
        .fold(init, |acc, line|
            acc.as_slice()
                .windows(2)
                .map(|window| window.iter().max().unwrap())
                .zip(line.iter())
                .map(|pair| pair.0 + pair.1)
                .collect())
        .get(0).unwrap().clone();

    println!("Answer: {}", answer);
}