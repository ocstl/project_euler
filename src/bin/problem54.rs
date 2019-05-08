use project_euler::cards::poker::Hand;
use std::fs;

const INPUT: &str = "inputs/p054_poker.txt";

fn line_to_hands(line: &str) -> (Hand, Hand) {
    let mut cards = line.split_whitespace();

    let hand1 = Hand::from([
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
    ]);

    let hand2 = Hand::from([
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
    ]);

    (hand1, hand2)
}

fn main() {
    let data = fs::read_to_string(INPUT).expect("File not found.");
    let answer = data
        .lines()
        .map(line_to_hands)
        .filter(|(hand1, hand2)| hand1 > hand2)
        .count();

    println!("Answer: {}", answer);
}
