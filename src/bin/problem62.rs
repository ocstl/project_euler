use project_euler::unsigned::UnsignedInteger;
use std::collections::HashMap;

const BASE: u64 = 10;
const PERMUTATIONS: usize = 5;
const POWER: u32 = 3;

/// Find the smallest cube for which exactly five permutations of its digits are cube.
fn main() {
    let mut counter: HashMap<Vec<u64>, Vec<u64>> = HashMap::new();

    // Map the cubes to an ordered vector of the digits, to be used as a key. Could use a u64 as
    // a key instead, but keep in mind that 0's will disappear unless one generates the largest
    // number (`from_radix_le` would work).
    let ordered_digits = |n: u64| -> Vec<u64> {
        let mut v = n.to_radix_le(BASE);
        v.sort();
        v
    };

    let answer: u64 = (1..)
        .map(|n: u64| n.pow(POWER))
        .find_map(|n| {
            let v = ordered_digits(n);
            let c = counter.entry(v).or_insert(Vec::new());
            c.push(n);

            // Since we generated cubes in order, the first in the series is the smallest one.
            if c.len() == PERMUTATIONS {
                Some(*c.first().unwrap())
            }
            else {
                None
            }
        })
        .unwrap_or(0);

    println!("The answer is: {}", answer);
}
