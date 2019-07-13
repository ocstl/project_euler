use primal::Sieve;

const INPUT: usize = 4;
const LARGEST_PRIME: usize = 10_000_000;

/// Find the first four consecutive integers to have four distinct prime factors each. What is the
/// first of these numbers?
fn main() {
    let sieve = Sieve::new(LARGEST_PRIME);
    let nbr_distinct_prime_factors = |n: usize| -> usize {
        sieve.factor(n).unwrap().len()
    };

    let mut current = 2;
    let mut nbr_factors = Vec::with_capacity(INPUT);

    for _ in 0..INPUT {
        nbr_factors.push(nbr_distinct_prime_factors(current));
        current += 1;
    }

    // Keep adding new numbers until we hit the required number of consecutive numbers with the
    // right amount of distinct prime factors.
    while !nbr_factors.iter().all(|&n| n == INPUT) {
        nbr_factors.remove(0);
        nbr_factors.push(nbr_distinct_prime_factors(current));
        current += 1;
    }

    println!("Answer: {}", current - INPUT);
}
