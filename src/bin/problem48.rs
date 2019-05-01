const INPUT: u64 = 1000;

// Reasonably fast for our purposes, but could be improved by using exponents to the power of 2.
fn mod_pow(n: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    for _ in 0..exponent {
        result = (result * n) % modulus
    }

    result
}

fn main() {
    let modulus = 10u64.pow(10);

    // Get the last ten digits of the sum of 1^1 to n^n.
    let answer: u64 = (1..=INPUT).map(|n| mod_pow(n, n, modulus)).sum::<u64>() % modulus;

    println!("Answer: {}", answer);
}
