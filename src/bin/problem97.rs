const BASE: usize = 28433;
const EXPONENT: usize = 7830457;
const FILTER: usize = 10_000_000_000;

/// The first known prime found to exceed one million digits was discovered in 1999, and is a
/// Mersenne prime of the form 26972593−1; it contains exactly 2,098,960 digits. Subsequently
/// other Mersenne primes, of the form 2p−1, have been found which contain more digits.
///
/// However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits:
/// 28433×27830457+1.
///
/// Find the last ten digits of this prime number.
fn main() {
    let answer = (0..EXPONENT).fold(BASE, |acc, _| (acc << 1) % FILTER) + 1;
    println!("The answer is: {}", answer);
}
