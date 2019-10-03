use radixal::IntoDigits;
use std::collections::{HashMap, HashSet};

const INPUT: u64 = 1_000_000;
const EXACT_LENGTH: usize = 60;

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn sum_factorial(n: u64) -> u64 {
    n.into_decimal_digits().map(factorial).sum()
}

/// The number 145 is well known for the property that the sum of the factorial of its digits is
/// equal to 145:
///
/// 1! + 4! + 5! = 1 + 24 + 120 = 145
///
/// It is not difficult to prove that EVERY starting number will eventually get stuck in a loop.
/// How many chains, with a starting number below one million, contain exactly sixty
/// non-repeating terms?
fn main() {
    // Keep track of the lengths we've already discovered.
    let mut chain_lengths: HashMap<u64, usize> = HashMap::new();

    let chain_length = move |n: u64| -> usize {
        // Using a `Vec` would allow us to fill the `HashMap` faster, but lookups would be more
        // costly, not counting some legerdemain for special cases like 145.
        let mut visited = HashSet::new();
        let mut count = 0;
        let mut m = n;

        loop {
            if let Some(&l) = chain_lengths.get(&m) {
                count += l;
                chain_lengths.insert(n, count);
                break;
            } else if visited.contains(&m) {
                chain_lengths.insert(n, count);
                break;
            }

            visited.insert(m);
            let old = m;
            m = sum_factorial(m);

            // Do not count a step if the number remains unchanged.
            count += if old != m { 1 } else { 0 }
        }

        count
    };

    let answer = (1..=INPUT)
        .map(chain_length)
        .filter(|&length| length == EXACT_LENGTH)
        .count();

    println!("The answer is: {}", answer);
}
