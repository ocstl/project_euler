const LIMIT: usize = 1_000_000;
const TARGET: usize = 10;

/// Given the positive integers, x, y, and z, are consecutive terms of an
/// arithmetic progression, the least value of the positive integer, n, for
/// which the equation, x^2 − y^2 − z^2 = n, has exactly two solutions is n = 27:
///
/// 34^2 − 27^2 − 20^2 = 12^2 − 9^2 − 6^2 = 27
///
/// It turns out that n = 1155 is the least value which has exactly ten solutions.
///
/// How many values of n less than one million have exactly ten distinct solutions?
fn main() {
    // Defining x = y + a and z = y - a, some manipulation yields that:
    // y * (4a - y) = n
    // 4a - y = n / y
    // So, 4a > y and y is a factor of n.
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
