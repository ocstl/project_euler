use project_euler::fibonacci::Fibonacci;

const INPUT: usize = 12;

/// Consider the isosceles triangle with base length, b = 16, and legs, L = 17.
///
/// By using the Pythagorean theorem it can be seen that the height of the
/// triangle, h = sqrt(17^2 − 8^2) = 15, which is one less than the base length.
///
/// With b = 272 and L = 305, we get h = 273, which is one more than the base
/// length, and this is the second smallest isosceles triangle with the property
/// that h = b ± 1.
///
/// Find ∑ L for the twelve smallest isosceles triangles for which h = b ± 1
/// and b, L are positive integers.
fn main() {
    // Some arithmetic (using Heron's formula) yields:
    // L = sqrt( 5 * b^2 / 4 + 2b + 1) if h = b + 1
    // L = sqrt( 5 * b^2 / 4 - 2b + 1) if h = b - 1.
    // From this, we can see that b has to be even to eliminate the fractional
    // part. Replacing with 2c = b, we can simplify a bit:
    // L = sqrt( 5 * c^2 ± 4c + 1)
    // This allows to find out that the solutions are given by the sequence:
    // A(n) = Fibonacci(6n + 3) / 2 (see https://oeis.org/A007805).
    let answer: u64 = Fibonacci::<u64>::new()
        // +3
        .skip(2)
        // 6n
        .step_by(6)
        // Don't want the first one (n == 0).
        .skip(1)
        .map(|n| n / 2)
        .take(INPUT)
        .sum();

    println!("The answer is: {}", answer);
}
