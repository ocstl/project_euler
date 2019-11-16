use radixal::IntoDigits;
use std::collections::HashSet;

const LIMIT: u64 = 100_000_000;

fn sum_consecutive_squares(start: u64) -> impl Iterator<Item = u64> {
    (start + 1..)
        .scan(start * start, |state, n| {
            *state += n * n;
            Some(*state)
        })
        .take_while(|&s| s < LIMIT)
}

/// The palindromic number 595 is interesting because it can be written as the
/// sum of consecutive squares: 62 + 72 + 82 + 92 + 102 + 112 + 122.
///
/// There are exactly eleven palindromes below one-thousand that can be written
/// as consecutive square sums, and the sum of these palindromes is 4164. Note
/// that 1 = 02 + 12 has not been included as this problem is concerned with the
/// squares of positive integers.
///
/// Find the sum of all the numbers less than 108 that are both palindromic and
/// can be written as the sum of consecutive squares.
fn main() {
    let answer: u64 = (1..)
        .take_while(|&n| 2 * n * n < LIMIT)
        .flat_map(sum_consecutive_squares)
        .filter(|&n| n.is_decimal_palindrome())
        // Some sums are found multiple times.
        .collect::<HashSet<u64>>()
        .iter()
        .sum();

    println!("The answer is: {}", answer);
}
