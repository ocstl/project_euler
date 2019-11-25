use primal::Sieve;
use radixal::IntoDigits;

const FIRST_PRIME: usize = 5;
const LAST_PRIME: usize = 1_000_003;

fn mod_pow(a: usize, mut exponent: usize, modulus: usize) -> usize {
    let mut result = 1;
    let mut base = a % modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result * base % modulus;
        }
        exponent >>= 1;
        base = base * base % modulus;
    }

    result
}

/// Consider the consecutive primes p1 = 19 and p2 = 23. It can be verified
/// that 1219 is the smallest number such that the last digits are formed by p1
/// whilst also being divisible by p2.
///
/// In fact, with the exception of p1 = 3 and p2 = 5, for every pair of
/// consecutive primes, p2 > p1, there exist values of n for which the last
/// digits are formed by p1 and n is divisible by p2. Let S be the smallest of
/// these values of n.
///
/// Find ∑ S for every pair of consecutive primes with 5 ≤ p1 ≤ 1000000.
fn main() {
    // Splitting things up:
    // k * 10^n + p1 == 0 mod p2
    // where n is the number of decimal digits in p1. We need to find k.
    //
    // By Euler's theorem, the multiplicative inverse of 10^n is:
    // a^-1 == (10^n)^(p - 2) mod p2, and
    // a^-1 * 10^n == 1 mod p2.
    //
    // So:
    // (p2 - p1) * a^-1 * 10^n == (p2 - p1) mod p2.
    //
    // To get the smallest "offset" from p1:
    // ((p2 - p1) * a^-1) % p2) * 10^n
    let primes = Sieve::new(LAST_PRIME);
    let first_primes = primes.primes_from(FIRST_PRIME);
    let second_primes = primes.primes_from(FIRST_PRIME + 2);

    let answer: usize = first_primes
        .zip(second_primes)
        .map(|(p1, p2)| {
            let base = 10_usize.pow(p1.nbr_decimal_digits() as u32);
            let inverse = mod_pow(base, p2 - 2, p2);
            base * (inverse * (p2 - p1) % p2) + p1
        })
        .sum();

    println!("The answer is: {}", answer);
}
