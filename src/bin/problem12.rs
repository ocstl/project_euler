use primal::Sieve;
use project_euler::polygonal_numbers::PolygonalNumberIterator;

const INPUT: usize = 500;
const LARGEST_PRIME: usize = 10_000_000;

/// What is the value of the first triangle number to have over five hundred divisors?
fn main() {
    let sieve = Sieve::new(LARGEST_PRIME);

    // To get the number of factors, decompose into powers of primes. The product of the
    // exponents (adding 1 to account for 0) yields the number of factors.
    let nbr_factors =
        |n: usize| -> usize { sieve.factor(n).unwrap().iter().map(|v| v.1 + 1).product() };

    // We can use the product theorem to count the number of combinations of prime factors.
    let answer: usize = PolygonalNumberIterator::new(3)
        .find(|&n| nbr_factors(n) > INPUT)
        .unwrap_or(0);

    println!("Answer: {}", answer);
}
