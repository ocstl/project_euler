use primal::Primes;

const TARGET: usize = 10_000_000_000;

/// Let pn be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when (p_n − 1)^n +
/// (p_n + 1)^n is divided by p_n^2.
///
/// For example, when n = 3, p_3 = 5, and 43 + 63 = 280 ≡ 5 mod 25.
///
/// The least value of n for which the remainder first exceeds 10^9 is 7037.
///
/// Find the least value of n for which the remainder first exceeds 10^10.
fn main() {
    // ((p_n − 1)^n + (p_n + 1)^n) % p_n^2 yields 2 if n is even, 2 * n * p_n mod p_n^2 when n is
    // odd.
    let answer = Primes::all()
        .enumerate()
        .step_by(2)
        .skip_while(|(_, p)| p * p < TARGET)
        .find_map(|(n, p)| {
            let n = n + 1;
            if (2 * n * p) % (p * p) > TARGET {
                Some(n)
            } else {
                None
            }
        })
        .unwrap();

    println!("The answer is: {}", answer);
}
