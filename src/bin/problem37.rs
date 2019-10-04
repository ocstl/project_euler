use primal::Sieve;
use radixal::IntoDigits;
use std::iter::from_fn;

/// Find the sum of the only eleven primes that are both truncatable from left to right and right
/// to left.
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
fn main() {
    // The largest such prime is 748317, so 750_000 is fine.
    let primes = Sieve::new(750_000);

    let truncatable = |x: usize| -> bool {
        let mut digits = x.into_decimal_digits();
        let mut digits_left = digits.clone();

        let from_left =
            from_fn(move || Some(digits_left.to_number()).filter(|_| digits_left.next().is_some()))
                .skip(1);
        let from_right =
            from_fn(move || Some(digits.to_number()).filter(|_| digits.next_back().is_some()))
                .skip(1);

        from_left.chain(from_right).all(|n| primes.is_prime(n))
    };

    // As 2, 3, 5 and 7 are not truncatable, they are not included in the truncatable primes.
    let answer = primes
        .primes_from(10)
        .filter(|&x| truncatable(x))
        .take(11)
        .sum::<usize>();

    println!("Answer: {}", answer);
}
