use project_euler::primes::Primes;

fn main() {
    /* The largest such prime is 739397, so 750_000 is fine, at least until the next generation
     * of the Primes struct. */
    let primes: Vec<u32> =
        Primes::<u32>::new().take_while(|&x| x < 750_000).collect();

    let truncatable = |x: u32| -> bool {
        let mut y = x;
        while y > 0 {
            if primes.binary_search(&y).is_ok() { y /= 10 }
            else { return false}
        }

        let mut length = f64::from(x).log10() as u32;
        while length > 0 {
            if primes.binary_search(&(x % 10u32.pow(length))).is_ok() { length -= 1 }
            else { return false }
        }

        true
    };

    /* As 2, 3, 5 and 7 are not truncatable, they are not included in the truncatable primes. */
    let answer: u32 = primes.iter().skip_while(|&&x| x < 10)
        .filter(|&&x| truncatable(x))
        .take(11)
        .sum();

    println!("Answer: {}", answer);
}