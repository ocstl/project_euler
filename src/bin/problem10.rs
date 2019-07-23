use primal::Sieve;

const INPUT: usize = 2_000_000;

fn main() {
    let answer: usize = Sieve::new(INPUT)
        .primes_from(0)
        .take_while(|&x| x < INPUT)
        .sum();

    println!("Answer: {}", answer);
}
