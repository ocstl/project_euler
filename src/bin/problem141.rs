use num::integer::Roots;
use std::collections::HashSet;

const TARGET: u64 = 1_000_000_000_000;

/// A positive integer, n, is divided by d and the quotient and remainder are q
/// and r respectively. In addition d, q, and r are consecutive positive integer
/// terms in a geometric sequence, but not necessarily in that order.
///
/// For example, 58 divided by 6 has quotient 9 and remainder 4. It can also be
/// seen that 4, 6, 9 are consecutive terms in a geometric sequence
/// (common ratio 3/2).
/// We will call such numbers, n, progressive.
///
/// Some progressive numbers, such as 9 and 10404 = 102^2, happen to also be
/// perfect squares. The sum of all progressive perfect squares below one
/// hundred thousand is 124657.
///
/// Find the sum of all progressive perfect squares below one trillion (10^12).
fn main() {
    // Without loss of generality, set n = dq + r, with r < d < q.
    // Let the ratio be a / b, and r = b^2 c, so that q is an integer:
    // - r = b^2 c
    // - d = abc
    // - q = a^2 c
    // So:
    // n = a^3 b c^2 + b^2 c
    // So, we know that a < n^(1/3) and b < a (to preserve the progression).
    let answer: u64 = (2..TARGET.cbrt())
        .flat_map(|a| {
            (1_u64..a)
                .flat_map(move |b| {
                    (1_u64..)
                        .map(move |c| a.pow(3) * b * c.pow(2) + b.pow(2) * c)
                        .take_while(|&n| n < TARGET)
                })
        })
        .filter(|&n| n.sqrt().pow(2) == n)
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum();

    println!("The answer is: {:?}", answer);
}
