use primal::Sieve;
use project_euler::unsigned::UnsignedInteger;

/// Find the sum of the only eleven primes that are both truncatable from left to right and right
/// to left.
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
fn main() {
    // The largest such prime is 739397, so 750_000 is fine.
    let primes = Sieve::new(750_000);

    let truncatable = |x: usize| -> bool {
        let mut y = x;
        while y > 0 {
            if primes.is_prime(y) {
                y /= 10
            } else {
                return false;
            }
        }

        let mut length = x.nbr_digits(10) as u32;
        while length > 0 {
            if primes.is_prime(x % 10_usize.pow(length)) {
                length -= 1
            } else {
                return false;
            }
        }

        true
    };

    // As 2, 3, 5 and 7 are not truncatable, they are not included in the truncatable primes.
    let answer = primes
        .primes_from(10)
        .filter(|&x| truncatable(x))
        .take(11)
        .sum::<usize>();

    println!("Answer: {}", answer);
}
