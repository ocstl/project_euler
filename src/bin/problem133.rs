use primal::Sieve;

const LIMIT: usize = 100_000;

/// Find the smallest repunit that is a multiple of `n`.
///
/// We need only keep track of the remainder, until it reaches 0.
fn smallest_repunit(n: usize) -> usize {
    let mut remainder = 1;
    let mut count = 1;

    while remainder != 0 {
        remainder = (remainder * 10 + 1) % n;
        count += 1;
    }

    count
}

/// A number consisting entirely of ones is called a repunit. We shall define
/// R(k) to be a repunit of length k; for example, R(6) = 111111.
///
/// Let us consider repunits of the form R(10^n).
///
/// Although R(10), R(100), or R(1000) are not divisible by 17, R(10000) is
/// divisible by 17. Yet there is no value of n for which R(10^n) will divide
/// by 19. In fact, it is remarkable that 11, 17, 41, and 73 are the only four
/// primes below one-hundred that can be a factor of R(10^n).
///
/// Find the sum of all the primes below one-hundred thousand that will never
/// be a factor of R(10^n).
fn main() {
    // R(k) is divisible by R(m) if m is a factor of k. So, R(10^n) will be
    // divisible by R(2^p * 5^q), for 0 <= p <= n and 0 <= q <= n.
    // Thus, if we find the smallest repunit that is a multiple of a prime p,
    // R(k), and k's only factors are 2 and 5, then p can divide R(10^n) with
    // n = max(p, q).
    let sieve = Sieve::new(LIMIT);

    let filter_fn = |n: usize| {
        // These can never generate a smallest repunit.
        if n == 2 || n == 5 {
            return true;
        }

        let k = smallest_repunit(n);
        sieve
            .factor(k)
            .unwrap()
            .into_iter()
            .any(|(p, _)| p != 2 && p != 5)
    };

    let answer: usize = sieve
        .primes_from(0)
        .take_while(|&p| p < LIMIT)
        .filter(|&p| filter_fn(p))
        .sum();

    println!("The answer is: {}", answer);
}
