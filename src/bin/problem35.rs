use primal::Sieve;
use project_euler::unsigned::UnsignedInteger;

const INPUT: usize = 1_000_000;

// Generate all rotations of the number (abc -> cab -> bca), in decimal format.
fn rotations(x: usize) -> Vec<usize> {
    // Can't properly use conversion to f64 for a usize.
    let length = x.nbr_digits(10) as u32;

    (0..length)
        .scan(x, |acc, _| {
            let div = *acc / 10;
            let rem = *acc % 10;
            *acc = div + rem * 10_usize.pow(length - 1);
            Some(*acc)
        })
        .collect()
}

/// The number, 197, is called a circular prime because all rotations of the digits: 197, 971,
/// and 719, are themselves prime. How many circular primes are there below one million?
fn main() {
    let primes = Sieve::new(INPUT);

    let answer = primes
        .primes_from(0)
        .filter(|&x| rotations(x).iter().all(|&x| primes.is_prime(x)))
        .count();

    println!("Answer: {}", answer);
}
