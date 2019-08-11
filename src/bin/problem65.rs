use num::{BigUint, One, Zero};
use project_euler::continued_fractions::ContinuedFraction;

const BASE: u32 = 10;
const INPUT: usize = 100;

// Generator of the partial denominators for the continued fraction representation of e (Euler's
// constant).
fn partial_denominator_e() -> impl Iterator<Item = BigUint> {
    let mut idx = BigUint::zero();
    let mut n = BigUint::zero();

    std::iter::from_fn(move || {
        idx += BigUint::one();
        if idx.is_one() {
            Some(BigUint::from(2_u8))
        } else if (idx.clone() % BigUint::from(3_u8)).is_zero() {
            n += 2_u8;
            Some(n.clone())
        } else {
            Some(BigUint::one())
        }
    })
}

/// Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
fn main() {
    let convergent = ContinuedFraction::new(partial_denominator_e())
        .into_iter()
        .nth(INPUT - 1)
        .unwrap();

    let answer: usize = convergent
        .numer()
        .to_radix_le(BASE)
        .iter()
        .map(|&x| usize::from(x))
        .sum();

    println!("The answer is: {}", answer);
}
