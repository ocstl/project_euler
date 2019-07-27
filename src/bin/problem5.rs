use counter::Counter;
use primal::Sieve;

const INPUT: usize = 20;

/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
/// 20?
fn main() {
    let sieve = Sieve::new(INPUT);
    let factors = (2..INPUT).fold(Counter::new(), |acc: Counter<usize>, x| {
        acc | sieve.factor(x).unwrap().into_iter().collect()
    });

    let answer: u64 = factors
        .iter()
        .map(|(key, value)| (*key as u64).pow(*value as u32))
        .product();

    println!("Answer: {}", answer);
}
