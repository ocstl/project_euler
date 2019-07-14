use primal::is_prime;

/// What is the smallest odd composite that cannot be written as the sum of a prime and twice a
/// square?
fn main() {
    let answer = (5u64..)
        .step_by(2)
        .find(|&n| {
            !is_prime(n)
                && (1..)
                    .take_while(|&x| 2 * (x * x) < n)
                    .all(|x| !is_prime(n - 2 * (x * x)))
        })
        .unwrap();

    println!("Answer: {}", answer);
}
