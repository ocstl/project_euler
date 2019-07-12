extern crate counter;

use counter::Counter;
use project_euler::factors;
use project_euler::polygonal_numbers::PolygonalNumberIterator;

const INPUT: usize = 500;

/// What is the value of the first triangle number to have over five hundred divisors?
fn main() {
    let nbr_factors = |n: usize| -> usize {
        factors::factorize(n)
            .iter()
            .collect::<Counter<_>>()
            .into_map()
            .values()
            .map(|v| v + 1)
            .product()
    };

    // We can use the product theorem to count the number of combinations of prime factors.
    let answer: usize = PolygonalNumberIterator::new(3)
        .find(|&n| nbr_factors(n) > INPUT)
        .unwrap_or(0);

    println!("Answer: {}", answer);
}
