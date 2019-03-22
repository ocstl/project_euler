use project_euler::primes::Primes;

const INPUT: usize = 10001;

fn main() {
    let answer: usize = Primes::new().nth(INPUT - 1).unwrap();

    println!("Answer: {}", answer);
}
