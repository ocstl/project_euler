use primal::Sieve;
use std::collections::HashSet;

const LARGEST_SUM: usize = 28123;

fn sum_proper_divisors(prime_factors: &[(usize, usize)]) -> usize {
    let s = |(k, v): &(usize, usize)| -> usize { (0..=*v as u32).map(|x| k.pow(x)).sum() };
    prime_factors.iter().map(s).product()
}

/// Find the sum of all the positive integers which cannot be written as the sum of two abundant
/// numbers.
fn main() {
    let sieve = Sieve::new(LARGEST_SUM);

    let wrapper = |n: usize| -> usize {
        let prime_factors = sieve.factor(n).unwrap();
        sum_proper_divisors(&prime_factors) - n
    };

    let abundant_numbers: Vec<usize> = (12..LARGEST_SUM)
        .filter(|&x| wrapper(x) > x)
        .collect();

    let sum_two_abundant_numbers: HashSet<usize> = abundant_numbers
        .iter()
        .flat_map(|x| abundant_numbers.iter().map(move |y| *x + y))
        .collect();

    let answer: usize = (1..=LARGEST_SUM)
        .collect::<HashSet<usize>>()
        .difference(&sum_two_abundant_numbers)
        .sum();

    println!("Answer: {}", answer);
}
