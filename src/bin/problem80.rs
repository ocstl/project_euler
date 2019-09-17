use num::bigint::BigUint;
use num::integer::Roots;
use num::traits::Pow;

const BASE: u32 = 10;
const INPUT: u64 = 100;
const PRECISION: usize = 100;

fn sum_square_root_digits(n: u64) -> u64 {
    (BigUint::from(n) * BigUint::from(BASE * BASE).pow(PRECISION))
        .sqrt()
        .to_radix_be(BASE)
        .iter()
        .take(PRECISION)
        .map(|&n| u64::from(n))
        .sum()
}

/// It is well known that if the square root of a natural number is not an integer, then it is
/// irrational. The decimal expansion of such square roots is infinite without any repeating
/// pattern at all.
///
/// The square root of two is 1.41421356237309504880..., and the digital sum of the first one
/// hundred decimal digits is 475.
///
/// For the first one hundred natural numbers, find the total of the digital sums of the first one
/// hundred decimal digits for all the irrational square roots.
fn main() {
    // Need to filter out the perfect squares.
    let answer: u64 = (1..=INPUT).filter_map(|n|
        if n.sqrt().pow(2) == n {
            None
        } else {
            Some(sum_square_root_digits(n))
        }).sum();

    println!("The answer is: {}", answer);
}

#[test]
fn test_sum_decimals() {
    assert_eq!(sum_square_root_digits(2), 475);
    assert_eq!(sum_square_root_digits(3), 441);
    assert_eq!(sum_square_root_digits(4), 2);
}
