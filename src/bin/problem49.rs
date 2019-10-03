use itertools::Itertools;
use primal::Primes;
use radixal::IntoDigits;
use std::collections::HashMap;

const BASE: usize = 10;
const INPUT: u32 = 4;
const SEQUENCE_LENGTH: usize = 3;

fn get_prime_permutations(nbr_digits: u32) -> HashMap<usize, Vec<usize>> {
    let mut permutations: HashMap<usize, Vec<usize>> = HashMap::new();

    // Transform the primes to the smallest usize formed with their digits, which will allow us
    // to group permutations together.
    let key_fn = |n: usize| -> usize {
        n.into_decimal_digits()
            .sorted()
            .fold(0, |acc, digit| acc * BASE + digit)
    };

    // Only use the primes with the correct number of digits.
    let primes = Primes::all()
        .filter(|&p| p >= 10usize.pow(nbr_digits - 1))
        .take_while(|&p| p < 10usize.pow(nbr_digits));

    for prime in primes {
        permutations.entry(key_fn(prime)).or_default().push(prime);
    }

    permutations
}

/// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is
/// unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
/// numbers are permutations of one another.
///
/// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
/// property, but there is one other 4-digit increasing sequence.
///
/// What 12-digit number do you form by concatenating the three terms in this sequence?
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
