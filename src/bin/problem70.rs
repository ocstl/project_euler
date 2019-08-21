use project_euler::totient;
use project_euler::unsigned::UnsignedInteger;

const BASE_10: usize = 10;
const INPUT: usize = 10_000_000;

/// Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n / φ(n)
/// produces a minimum.
fn main() {
    let f = totient::totient_function(INPUT);

    let answer = (2..INPUT)
        .filter_map(|n| match f(n) {
            Ok(t) if n.is_permutation(t, BASE_10) => Some((n as f64 / t as f64, n)),
            _ => None,
        })
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}
