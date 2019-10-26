use itertools::Itertools;
use permutohedron::LexicalPermutation;
use primal::Sieve;
use std::collections::HashSet;

const BASE: usize = 10;
const NBR_DIGITS: usize = 10;

/// Considering 4-digit primes containing repeated digits it is clear that they cannot all be the
/// same: 1111 is divisible by 11, 2222 is divisible by 22, and so on. But there are nine 4-digit
/// primes containing three ones:
///
/// 1117, 1151, 1171, 1181, 1511, 1811, 2111, 4111, 8111
///
/// We shall say that M(n, d) represents the maximum number of repeated digits for an n-digit
/// prime where d is the repeated digit, N(n, d) represents the number of such primes, and S(n,
/// d) represents the sum of these primes.
///
/// So M(4, 1) = 3 is the maximum number of repeated digits for a 4-digit prime where one is the
/// repeated digit, there are N(4, 1) = 9 such primes, and the sum of these primes is S(4, 1) =
/// 22275. It turns out that for d = 0, it is only possible to have M(4, 0) = 2 repeated digits,
/// but there are N(4, 0) = 13 such cases.
///
/// For d = 0 to 9, the sum of all S(4, d) is 273700.
///
/// Find the sum of all S(10, d).
fn main() {
    // Generating all primes takes way too much time. So we'll generate all numbers instead, then
    // check them for primeness.
    let sieve = Sieve::new(10_usize.pow(NBR_DIGITS as u32));

    let answer: usize = (0..=9)
        .filter_map(|digit| -> Option<usize> {
            (0..=NBR_DIGITS).rev().find_map(|repetitions| {
                let numbers: Vec<usize> = possible_numbers(digit, repetitions)
                    .into_iter()
                    .filter(|&n| sieve.is_prime(n))
                    .collect();

                if !numbers.is_empty() {
                    Some(numbers.into_iter().sum())
                } else {
                    None
                }
            })
        })
        .sum();

    println!("The answer is: {}", answer);
}

fn to_number(digits: &[usize]) -> usize {
    digits.iter().fold(0, |acc, digit| acc * BASE + digit)
}

fn possible_numbers(digit: usize, repetitions: usize) -> HashSet<usize> {
    let (single_digit, others): (Vec<usize>, Vec<usize>) = (0..=9).partition(|&d| d == digit);
    let digit_groups = std::iter::repeat(single_digit)
        .take(repetitions)
        .chain(std::iter::repeat(others).take(NBR_DIGITS - repetitions))
        .multi_cartesian_product();

    let mut result = HashSet::new();

    for mut group in digit_groups {
        // `Permutohedron` returns permutations in lexicographical order, so we need to start
        // with the first ordering.
        group.sort();
        loop {
            if group.first() != Some(&0) {
                result.insert(to_number(&group));
            }

            if !group.next_permutation() {
                break;
            }
        }
    }

    result
}
