use primal::Sieve;

const INPUT: usize = 100_000;
const INDEX: usize = 10_000;

thread_local! {
    static SIEVE: Sieve = Sieve::new(INPUT);
}

fn radical(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        SIEVE.with(|s| s.factor(n).unwrap().into_iter().map(|(p, _)| p).product())
    }
}

/// The radical of n, rad(n), is the product of the distinct prime factors of n. For example, 504
/// = 2^3 × 3^2 × 7, so rad(504) = 2 × 3 × 7 = 42.
///
/// If we calculate rad(n) for 1 ≤ n ≤ 10, then sort them on rad(n), and sorting on n if the
/// radical values are equal, we get:
/// [omitted]
///
/// Let E(k) be the kth element in the sorted n column; for example, E(4) = 8 and E(6) = 9.
///
/// If rad(n) is sorted for 1 ≤ n ≤ 100000, find E(10000).
fn main() {
    let mut values: Vec<usize> = (1..=INPUT).collect();
    values.sort_by_cached_key(|&n| radical(n));

    let answer = values[INDEX - 1];
    println!("The answer is: {}", answer);
}
