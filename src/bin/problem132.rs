use num::bigint::BigUint;
use num::traits::One;
use primal::Primes;

const EXPONENT: u64 = 1_000_000_000;
const NBR_PRIMES: usize = 40;

/// A number consisting entirely of ones is called a repunit. We shall define
/// R(k) to be a repunit of length k.
///
/// For example, R(10) = 1111111111 = 11×41×271×9091, and the sum of these prime
/// factors is 9414.
///
/// Find the sum of the first forty prime factors of R(109).
fn main() {
    let base = BigUint::from(10_u64);
    let exponent = BigUint::from(EXPONENT);

    // R(k) = (10^k - 1) / 9.
    // So, R(k) is divisible by p if 10^k = 1 mod 9p.
    let answer: usize = Primes::all()
        .filter(|&p| {
            base.clone()
                .modpow(&exponent, &BigUint::from(9 * p))
                .is_one()
        })
        .take(NBR_PRIMES)
        .sum();

    println!("The answer is: {}", answer);
}
