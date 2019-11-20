const INPUT: u64 = 1_000_000;

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
/// The least value of n for which A(n) first exceeds ten is 17.
///
/// Find the least value of n for which A(n) first exceeds one-million.
fn main() {
    // The requirement that GCD(n, 10) == 1 means we can skip even numbers, as
    // well as those divisible by 5.
    // Also, A(n) < n, so n is greater than 1_000_000.
    let answer = ((INPUT | 1)..)
        .step_by(2)
        .find(|&n| n % 5 != 0 && smallest_repunit(n) > INPUT)
        .unwrap();

    println!("The answer is: {}", answer);
}
