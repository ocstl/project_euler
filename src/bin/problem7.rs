use primal::Primes;

const INPUT: usize = 10001;

/// What is the 10 001st prime number?
fn main() {
    let answer: usize = Primes::all().nth(INPUT - 1).unwrap();

    println!("Answer: {}", answer);
}
