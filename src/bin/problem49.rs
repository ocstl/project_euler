use itertools::Itertools;
use project_euler::primes::Primes;
use project_euler::tools::{to_digits, to_number};
use std::collections::HashMap;

const INPUT: u32 = 4;
const SEQUENCE_LENGTH: usize = 3;

fn get_prime_permutations(nbr_digits: u32) -> HashMap<usize, Vec<usize>> {
    let mut permutations: HashMap<usize, Vec<usize>> = HashMap::new();

    // Transform the primes to the smallest usize formed with their digits, which will allow us
    // to group permutations together.
    let key_fn = |n: usize| -> usize {
        let mut v = to_digits(n);
        v.sort();
        to_number(&v)
    };

    // Only use the primes with the correct number of digits.
    let primes = Primes::<usize>::new()
        .filter(|&p| p >= 10usize.pow(nbr_digits - 1))
        .take_while(|&p| p < 10usize.pow(nbr_digits));

    for prime in primes {
        permutations.entry(key_fn(prime)).or_default().push(prime);
    }

    permutations
}

fn main() {
    let permutations = get_prime_permutations(INPUT);

    let sequences: Vec<String> = permutations
        .values()
        .flat_map(|primes| primes.iter().combinations(SEQUENCE_LENGTH))
        .filter_map(|combination| {
            // Check if we have an arithmetic sequence of the required length, and return a
            // String if this is the case.
            if combination
                .windows(2)
                .map(|window| window[1] - window[0])
                .all_equal()
            {
                Some(combination.iter().map(|n| format!("{}", n)).collect())
            } else {
                None
            }
        })
        .collect();

    println!("Answer: {:?}", sequences);
}
