use primal::{is_prime, Primes};
use project_euler::unsigned::UnsignedInteger;

const LENGTH: usize = 5;
const UPPER_BOUND: usize = 1_000_000;

// Concatenate two numbers.
fn concatenate(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.nbr_digits(10) as u32) + b
}

fn is_pair_prime(a: usize, b: usize) -> bool {
    is_prime(concatenate(a, b) as u64) && is_prime(concatenate(b, a) as u64)
}

#[derive(Clone, Debug)]
struct PrimePairSet {
    primes: Vec<usize>,
}

impl PrimePairSet {
    fn new(prime: usize) -> Self {
        PrimePairSet {
            primes: vec![prime],
        }
    }

    fn len(&self) -> usize {
        self.primes.len()
    }

    fn sum(&self) -> usize {
        self.primes.iter().sum()
    }

    fn add_if(&self, prime: usize) -> Option<Self> {
        if self.len() == LENGTH {
            None
        } else if self.primes.iter().all(|&p| is_pair_prime(p, prime)) {
            let mut s = self.clone();
            s.primes.push(prime);
            Some(s)
        } else {
            None
        }
    }
}

fn main() {
    let mut prime_pair_sets = Vec::new();
    let mut min_sum = UPPER_BOUND;

    for x in Primes::all() {
        // End the loop when any new prime is too large to improve the result.
        if x > min_sum {
            break;
        }

        let mut new_sets: Vec<PrimePairSet> = prime_pair_sets
            .iter()
            .filter_map(|s: &PrimePairSet| s.add_if(x))
            .collect();

        if let Some(i) = new_sets
            .iter()
            .filter(|s| s.len() >= LENGTH)
            .map(PrimePairSet::sum)
            .min()
        {
            min_sum = min_sum.min(i);
        }

        new_sets.push(PrimePairSet::new(x));
        prime_pair_sets.append(&mut new_sets);
    }

    let answer = prime_pair_sets
        .iter()
        .filter(|s| s.len() == LENGTH)
        .map(PrimePairSet::sum)
        .min()
        .unwrap_or(0);

    println!("Answer: {}", answer);
}
