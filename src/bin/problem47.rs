use project_euler::factors::factorize;

const INPUT: usize = 4;

fn main() {
    let nbr_distinct_prime_factors = |n: usize| -> usize {
        let mut prime_factors = factorize(n);
        prime_factors.dedup();
        prime_factors.len()
    };

    let mut current = 0;
    let mut nbr_factors = Vec::with_capacity(INPUT);

    for _ in 0..INPUT {
        nbr_factors.push(nbr_distinct_prime_factors(current));
        current += 1;
    }

    // Keep adding new numbers until we hit the required number of consecutive numbers with the
    // right amount of distinct prime factors.
    while !nbr_factors.iter().all(|&n| n == INPUT) {
        nbr_factors.remove(0);
        nbr_factors.push(nbr_distinct_prime_factors(current));
        current += 1;
    }

    println!("Answer: {}", current - INPUT);
}
