use itertools::Itertools;
use radixal::IntoDigits;

const MULTIPLES: [usize; 5] = [2, 3, 4, 5, 6];

fn multiple_permutation(n: usize, multiples: &[usize]) -> bool {
    let digits: Vec<usize> = n.into_decimal_digits().sorted().collect();

    multiples.iter().all(|&multiple| {
        let m: Vec<usize> = (n * multiple).into_decimal_digits().sorted().collect();
        m == digits
    })
}

/// It can be seen that the number, 125874, and its double, 251748, contain exactly the same
/// digits, but in a different order.
///
/// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
/// digits.
fn main() {
    let answer = (1..)
        .find(|&n| multiple_permutation(n, &MULTIPLES))
        .unwrap();

    println!("Answer: {}", answer);
}
