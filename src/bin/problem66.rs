use num::bigint::BigInt;
use num::rational::Ratio;
use num::traits::{One, Pow};
use project_euler::continued_fractions::square_root::SquareRoot;
use project_euler::continued_fractions::ContinuedFraction;

const INPUT: u32 = 1000;

/// Consider quadratic Diophantine equations of the form:
///     x2 – Dy2 = 1
///
/// For example, when D=13, the minimal solution in x is 6492 – 13×1802 = 1.
///
/// Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is
/// obtained.

// The fundamental solution consists in using the sequence of convergents of the square root of D
// to find the first pair (numerator, denominator) where the relation is satisfied.
fn main() {
    let filter_function = |d: u32| {
        move |r: &Ratio<BigInt>| -> bool {
            (r.numer().pow(2_u32) - d * r.denom().pow(2_u32)).is_one()
        }
    };

    let answer = (2..=INPUT)
        .filter_map(|d| {
            let f = filter_function(d);
            ContinuedFraction::from(SquareRoot::new(BigInt::from(d)))
                .into_iter()
                .find(f)
                // Map the numerator first, so the `max` function will yield this one.
                .map(|ratio| (ratio.numer().clone(), d))
        })
        .max()
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}
