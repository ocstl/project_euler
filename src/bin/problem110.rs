use lazy_static::lazy_static;
use primal::Primes;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const INPUT: usize = 4_000_000;

lazy_static! {
    static ref PRIMES: Vec<usize> = {
        let nbr_possible_primes = ((2 * INPUT - 1) as f64).log(3.0).ceil() as usize;
        Primes::all().take(nbr_possible_primes).collect()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Candidate {
    n: usize,
    xs: Vec<usize>,
}

impl Candidate {
    fn new(xs: &[usize]) -> Self {
        let n = PRIMES
            .iter()
            .zip(xs.iter())
            .map(|(&p, &a)| p.pow(a as u32))
            .product();
        Candidate { n, xs: xs.to_vec() }
    }

    fn nbr_divisors(&self) -> usize {
        self.xs.iter().map(|x| 2 * x + 1).product::<usize>()
    }

    fn increment(&self) -> Vec<Self> {
        (0..self.xs.len())
            .filter_map(|idx| {
                if idx == 0 || self.xs[idx] < self.xs[idx - 1] {
                    let mut new_xs = self.xs.clone();
                    new_xs[idx] += 1;
                    Some(Candidate::new(&new_xs))
                } else {
                    None
                }
            })
            .collect()
    }
}

// Implement a reverse PartialOrd, since we want the smallest solution.
impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.n.partial_cmp(&other.n).map(Ordering::reverse)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n.cmp(&other.n).reverse()
    }
}

/// In the following equation x, y, and n are positive integers.
///
/// 1 / x + 1 / y = 1 / n
///
/// It can be verified that when n = 1260 there are 113 distinct solutions and this is the least
/// value of n for which the total number of distinct solutions exceeds one hundred.
///
/// What is the least value of n for which the number of distinct solutions exceeds four million?
fn main() {
    // Building on the work for problem 108, since n = p1^a1 * p2^a2 * ..., where p is a prime
    // and a is its power, we need (2a1 + 1) * (2a2 + 1) * ... > 2 * INPUT - 1. Alternatively, we
    // could find the square of that number to simplify ( (a1 + 1) * (a2 + 2) * ...), then take the
    // square root.
    let nbr_possible_primes = ((2 * INPUT - 1) as f64).log(3.0).ceil() as usize;
    let mut candidates = BinaryHeap::new();
    let mut visited = HashSet::new();
    candidates.push(Candidate::new(&vec![0; nbr_possible_primes]));

    let mut answer = std::usize::MAX;
    while let Some(candidate) = candidates.pop() {
        if candidate.nbr_divisors() > 2 * INPUT - 1 {
            answer = candidate.n;
            break;
        }

        if visited.insert(candidate.n) {
            candidates.extend(candidate.increment().into_iter().filter(|c| c.n < answer));
        }
    }

    println!("The answer is: {}", answer);
}
