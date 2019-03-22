use project_euler::primes::Primes;

const INPUT: usize = 2000000;

fn main() {
    let answer: usize = Primes::<usize>::new().take_while(|&x| x < INPUT).sum();

    println!("Answer: {}", answer);
}
