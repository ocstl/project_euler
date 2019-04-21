use num::BigUint;
use std::collections::HashSet;
use std::iter::successors;

const INPUT: u32 = 100;

fn main() {
    /* We exponentiate from 2 to INPUT, thus we need INPUT - 1 elements. */
    let powers_of_a = move |a| {
        successors(Some(BigUint::from(a * a)), move |x| Some(x * a)).take(INPUT as usize - 1)
    };

    let answer = (2..=INPUT)
        .flat_map(powers_of_a)
        .collect::<HashSet<_>>()
        .len();
    println!("Answer: {}", answer);
}
