use project_euler::primes::Primes;

fn main() {
    let mut primes = Primes::new();

    // Find the first composite (non-prime) number that it is not the sum of a prime and the
    // square of a natural number.
    let answer = (5u32..)
        .step_by(2)
        .find(|&n| {
            !primes.check_primeness(n)
                && (1..)
                    .take_while(|&x| 2 * (x * x) < n)
                    .all(|x| !primes.check_primeness(n - 2 * (x * x)))
        })
        .unwrap();

    println!("Answer: {}", answer);
}
