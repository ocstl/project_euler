use permutohedron::Heap;
use primal::Sieve;

const BASE: usize = 10;
const DIGITS: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

thread_local! {
    static SIEVE: Sieve = Sieve::new(1_000_000_000);
}

fn is_prime(n: usize) -> bool {
    SIEVE.with(|s| s.is_prime(n))
}

/// Using all of the digits 1 through 9 and concatenating them freely to form decimal integers,
/// different sets can be formed. Interestingly with the set {2,5,47,89,631}, all of the elements
/// belonging to it are prime.
///
/// How many distinct sets containing each of the digits one through nine exactly once contain
/// only prime elements?
fn main() {
    let mut digits = DIGITS;
    let heap = Heap::new(&mut digits);

    let answer: usize = heap.map(|digits| count_sets(digits.to_vec(), 0)).sum();
    println!("The answer is: {}", answer);
}

// Recursive solution.
fn count_sets(mut digits: Vec<usize>, last_number: usize) -> usize {
    // If the first digit (which is the last we pick, as we `pop` the `Vec`) is even, the last
    // number will be even, thus not prime. The only exception would be 2, but we only accept
    // increasing numbers, so 2 can't be the last number.
    if digits.first().unwrap_or(&0) & 1 == 0 {
        return 0;
    }

    // If the last (first, since we're popping the vector) number is even, any set we build will
    // contain at least one even number, which will not be prime. The one exception would be 2,
    // but, since we use increasing numbers, 2 will never be the last number.
    let mut n = 0;
    let mut count = 0;
    while let Some(digit) = digits.pop() {
        n = n * BASE + digit;

        // Both the permutations 254789631 and 524789631 yield the set {2, 5, 47, 89, 631}. By
        // using only increasing numbers, we don't have to keep track of the previously yielded
        // sets.
        if n < last_number {
            continue;
        }

        if is_prime(n) {
            count += if digits.is_empty() {
                1
            } else {
                count_sets(digits.clone(), n)
            };
        }
    }

    count
}
