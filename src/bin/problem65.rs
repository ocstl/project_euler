use num::{BigInt, One, Zero};
use project_euler::continued_fractions::ConvergentIterator;

const BASE: u32 = 10;
const INPUT: usize = 100;

// Generator of the partial denominators for the continued fraction representation of e (Euler's
// constant).
fn partial_denominator_e() -> impl Iterator<Item = BigInt> {
    let mut idx = BigInt::zero();
    let mut n = BigInt::zero();

    std::iter::from_fn(move || {
        idx += BigInt::one();
        if idx.is_one() {
            Some(BigInt::from(2))
        } else if (idx.clone() % BigInt::from(3)).is_zero() {
            n += 2;
            Some(n.clone())
        } else {
            Some(BigInt::one())
        }
    })
}

/// Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
fn main() {
    let convergent = ConvergentIterator::new(partial_denominator_e())
        .nth(INPUT - 1)
        .unwrap();
    let answer: usize = convergent
        .numer()
        .to_radix_le(BASE)
        .1
        .iter()
        .map(|&x| usize::from(x))
        .sum();

    println!("The answer is: {}", answer);
}
