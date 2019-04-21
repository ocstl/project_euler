use project_euler::primes::Primes;

const INPUT: u32 = 1_000_000;

/* Generate all rotations of the number (abc -> cab -> bca), in decimal format. */
fn rotations(x: u32) -> Vec<u32> {
    let length = (f64::from(x)).log10().ceil() as u32;

    (0..length)
        .scan(x, |acc, _| {
            let div = *acc / 10;
            let rem = *acc % 10;
            *acc = div + rem * 10_u32.pow(length - 1);
            Some(*acc)
        })
        .collect()
}

fn main() {
    let primes: Vec<u32> = Primes::new().take_while(|&x| x < INPUT).collect();

    /* Binary search, instead of 'contains' seems to provide a slight improvement in speed. A
     * HashSet did not provide much improvement overall (most of the time is spent generating the
     * primes it seems). */
    let answer = primes
        .iter()
        .filter(|&&x| rotations(x).iter().all(|x| primes.binary_search(x).is_ok()))
        .count();

    println!("Answer: {}", answer);
}
