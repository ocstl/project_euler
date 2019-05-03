use project_euler::primes::Primes;

const INPUT: usize = 1_000_000;

fn main() {
    let mut primes = Primes::<usize>::new();
    let vec_primes: Vec<usize> = primes
        .clone()
        .take_while(|&prime| prime < INPUT)
        .collect();

    // Iterate over sequences lengths (windows).
    let answer: usize = (1..=vec_primes.len())
        .rev()
        .filter_map(|n| {
            vec_primes
                .windows(n)
                .map(|window| window.iter().sum())
                .take_while(|&s| s < INPUT)
                .filter(|&s| primes.check_primeness(s))
                .max()
        })
        .next()
        .unwrap();

    println!("Answer: {}", answer);
}
