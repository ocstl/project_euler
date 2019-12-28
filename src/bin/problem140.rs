use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const TARGET: usize = 30;

/// Consider the infinite polynomial series A_G(x) = x G_1 + x^2 G_2 + x^3 G_3 + ..., where
/// G_k is the kth term of the second order recurrence relation G_k = G_{k−1} + G_{k−2},
/// G_1 = 1 and G_2 = 4; that is, 1, 4, 5, 9, 14, 23, ....
///
/// For this problem we shall be concerned with values of x for which A_G(x) is
/// a positive integer.
///
/// We shall call A_G(x) a golden nugget if x is rational, because they become
/// increasingly rarer; for example, the 20th golden nugget is 211345365.
///
/// Find the sum of the first thirty golden nuggets.
fn main() {
    // The generating function for the sequence is given by:
    // A_G(x) = (3 x^2 + x) / ( 1 - x - x^2)
    // We're interested in cases where A_G(x) is an integer (n), which yields:
    // x^2 (n + 3) + x (n + 1) - n = 0
    // We can thus use the quadratic formula to find x. Given that n is an
    // integer, the denominator is an integer; for the numerator to be an
    // integer as well, the square root part (5 n^2 + 14 n + 1)  needs to be a
    // perfect square. In other words, a Diophantine equation:
    // 5 n^2 + 14 n + 1 - y^2 = 0
    // Some rewriting yields:
    // x^2 - 5 y^2 - 44 = 0, where x = (5n + 7)
    // which is much easier to work with.
    let mut candidates = BinaryHeap::new();
    candidates.push(Reverse((7, 1)));
    candidates.push(Reverse((8, 2)));
    candidates.push(Reverse((13, 5)));

    let mut nuggets = HashSet::new();
    let mut visited = HashSet::new();

    while let Some(Reverse((x, y))) = candidates.pop() {
        if !visited.insert((x, y)) {
            continue;
        }

        if nuggets.len() == TARGET {
            break;
        }

        // We don't want the solution at x == 7 (n == 0). Also, only integer
        // solutions are wanted (x % 5 == 2 is equivalent to (x - 7) % 5 == 0).
        if x != 7 && x % 5 == 2 {
            nuggets.insert((x - 7) / 5);
        }

        candidates.push(Reverse((9 * x - 20 * y, -4 * x + 9 * y)));
        candidates.push(Reverse((9 * x + 20 * y, 4 * x + 9 * y)));
    }

    let answer: i64 = nuggets.into_iter().sum();
    println!("The answer is: {}", answer);
}
