use primal::Sieve;

const INPUT: usize = 1_000_000;

/// Which prime, below one-million, can be written as the sum of the most consecutive primes?
fn main() {
    let sieve = Sieve::new(INPUT);
    let vec_primes: Vec<usize> = sieve.primes_from(0).collect();

    // Iterate over sequences lengths (windows).
    let answer: usize = (1..=vec_primes.len())
        .rev()
        .filter_map(|n| {
            vec_primes
                .windows(n)
                .map(|window| window.iter().sum())
                .take_while(|&s| s < INPUT)
                .filter(|&s| sieve.is_prime(s))
                .max()
        })
        .next()
        .unwrap();

    println!("Answer: {}", answer);
}
