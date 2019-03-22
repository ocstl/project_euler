use project_euler::primes::Primes;

const INPUT: u64 = 600_851_475_143;

fn main() {
    let mut primes = Primes::new();
    let mut number = INPUT;
    let mut current;
    let mut answer = 0;

    while number > 1 {
        current = primes.next().unwrap();
        while number % current == 0 {
            number /= current;
            answer = current;
        }
    }

    println!("Answer: {}", answer);
}
