use itertools::Itertools;
use radixal::IntoDigits;
use std::collections::HashMap;

const PERMUTATIONS: usize = 5;
const POWER: u32 = 3;

/// Find the smallest cube for which exactly five permutations of its digits are cube.
fn main() {
    let mut counter: HashMap<Vec<u64>, Vec<u64>> = HashMap::new();

    let answer: u64 = (1..)
        .map(|n: u64| n.pow(POWER))
        .find_map(|n| {
            // Map the cubes to an ordered vector of the digits, to be used as a key.
            let v = n.into_decimal_digits().sorted().collect();
            let c = counter.entry(v).or_insert_with(|| Vec::new());
            c.push(n);

            // Since we generated cubes in order, the first in the series is the smallest one.
            if c.len() == PERMUTATIONS {
                Some(*c.first().unwrap())
            } else {
                None
            }
        })
        .unwrap_or(0);

    println!("The answer is: {}", answer);
}
