use primal::is_prime;

const INPUT: usize = 25;

/// Find the smallest repunit that is a multiple of `n`.
///
/// We need only keep track of the remainder, until it reaches 0.
fn smallest_repunit(n: u64) -> u64 {
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
/// Given that n is a positive integer and GCD(n, 10) = 1, it can be shown that
/// there always exists a value, k, for which R(k) is divisible by n, and let
/// A(n) be the least such value of k; for example, A(7) = 6 and A(41) = 5.
///
/// You are given that for all primes, p > 5, that p − 1 is divisible by A(p).
/// For example, when p = 41, A(41) = 5, and 40 is divisible by 5.
///
/// However, there are rare composite values for which this is also true; the
/// first five examples being 91, 259, 451, 481, and 703.
///
/// Find the sum of the first twenty-five composite values of n for which
/// GCD(n, 10) = 1 and n − 1 is divisible by A(n).
fn main() {
    // Start at 91, since this is the first, skipping the even numbers and those
    // divisible by 5.
    let answer: u64 = (91..)
        .step_by(2)
        .filter(|&n| {
            n % 5 != 0 && !is_prime(n) && {
                let a = smallest_repunit(n);
                n % a == 1
            }
        })
        .take(INPUT)
        .sum();

    println!("The answer is: {}", answer);
}
