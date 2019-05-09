use core::iter::successors;
use project_euler::tools::ReverseInteger;

const INPUT: u128 = 10000;
const MAX_ITERATIONS: usize = 50;

fn is_lychrel_number(n: u128) -> bool {
    successors(Some(n + n.reverse_base10()), |n| {
        Some(n + n.reverse_base10())
    })
    .take(MAX_ITERATIONS)
    .any(|n| n.is_palindrome_base10())
}

fn main() {
    let answer = (1..INPUT).filter(|&n| !is_lychrel_number(n)).count();
    println!("Answer: {}", answer);
}
