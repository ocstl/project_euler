use primal::Sieve;

const INPUT: usize = 1_000_000;

/// Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
/// number of numbers less than n which are relatively prime to n.
///
/// Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
fn main() {
    let primes = Sieve::new(INPUT);
    let totient = |n: usize| -> usize {
        primes
            .factor(n)
            .unwrap()
            .into_iter()
            // See the proof of Euler's formula
            // (https://en.wikipedia.org/wiki/Euler%27s_totient_function#Proof_of_Euler's_product_formula)
            // Using a product of `f64` yields no appreciable improvement in performance.
            .map(|(p, x)| p.pow(x as u32) - p.pow(x as u32 - 1))
            .product()
    };

    let answer = (2..=INPUT)
        .map(|n| (n as f64 / totient(n) as f64, n))
        .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}
