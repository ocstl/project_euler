use permutohedron::Heap;
use project_euler::unsigned::UnsignedInteger;

const BASE: usize = 10;
const INPUT: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const PRIMES: [usize; 7] = [2, 3, 5, 7, 11, 13, 17];


/// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
/// digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility
/// property.
///
/// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
/// * d2d3d4=406 is divisible by 2
/// * d3d4d5=063 is divisible by 3
/// * d4d5d6=635 is divisible by 5
/// * d5d6d7=357 is divisible by 7
/// * d6d7d8=572 is divisible by 11
/// * d7d8d9=728 is divisible by 13
/// * d8d9d10=289 is divisible by 17
///
/// Find the sum of all 0 to 9 pandigital numbers with this property.
fn main() {
    let mut digits = INPUT;
    let heap = Heap::new(&mut digits);
    let to_number = |x: &[usize]| usize::from_radix_be(x.into_iter().cloned(), BASE);

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
