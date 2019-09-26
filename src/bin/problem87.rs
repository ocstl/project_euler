use primal::Primes;
use std::collections::HashSet;

const INPUT: usize = 50_000_000;

/// How many numbers below fifty million can be expressed as the sum of a prime square, prime
/// cube, and prime fourth power?
fn main() {
    let squares = prime_powers(2);
    let cubes = prime_powers(3);
    let fourths = prime_powers(4);

    // Brute force seems fine.
    let answer = squares
        .iter()
        .flat_map(|square| cubes.iter().map(move |cube| square + cube))
        .flat_map(|s| fourths.iter().map(move |fourth| s + fourth))
        .filter(|&s| s < INPUT)
        .collect::<HashSet<usize>>()
        .len();

    println!("The answer is: {}", answer);
}

fn prime_powers(power: u32) -> Vec<usize> {
    Primes::all()
        .map(move |p| p.pow(power))
        .take_while(|&p| p < INPUT)
        .collect()
}
