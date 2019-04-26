use permutohedron::Heap;
use project_euler::tools::to_number;

const PRIMES: [usize; 7] = [2, 3, 5, 7, 11, 13, 17];
const INPUT: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let mut digits = INPUT;
    let heap = Heap::new(&mut digits);

    // Check divisibility over all 3-digit windows, starting at the second digit.
    let filter_fn = |d: &[usize]| -> bool {
        d.windows(3)
            /* Start at the second digit. */
            .skip(1)
            .map(to_number)
            .zip(PRIMES.iter())
            .all(|(number, divisor)| number % divisor == 0)
    };

    let answer: usize = heap
        .filter_map(|permutation| {
            if filter_fn(&permutation) {
                Some(to_number(&permutation))
            } else {
                None
            }
        })
        .sum();

    println!("Answer: {}", answer);
}
