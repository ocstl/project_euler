const LIMIT: usize = 50_000_000;
const TARGET: usize = 1;

/// The positive integers, x, y, and z, are consecutive terms of an arithmetic
/// progression. Given that n is a positive integer, the equation,
/// x2 − y2 − z2 = n, has exactly one solution when n = 20:
///
/// 13^2 − 10^2 − 7^2 = 20
///
/// In fact there are twenty-five values of n below one hundred for which the
/// equation has a unique solution.
///
/// How many values of n less than fifty million have exactly one solution?
fn main() {
    // We can reuse problem 135, but with a larger `LIMIT` and a different
    // `TARGET`.
    let mut counts = vec![0; LIMIT as usize];

    for y in 1..LIMIT {
        let lower = (y + 3) / 4;

        for a in lower..y {
            let n = (4 * a - y) * y;
            if n >= LIMIT {
                break;
            } else {
                counts[n] += 1;
            }
        }
    }

    let answer = counts.into_iter().filter(|&c| c == TARGET).count();

    println!("The answer is: {}", answer);
}
