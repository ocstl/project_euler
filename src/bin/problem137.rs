use project_euler::fibonacci::Fibonacci;

const TARGET: usize = 15;

/// Consider the infinite polynomial series AF(x) = xF1 + x^2 F2 + x^3 F3 + ...,
/// where Fk is the kth term in the Fibonacci sequence: 1, 1, 2, 3, 5, 8, ...;
/// that is, Fk = F{k−1} + F{k−2}, F1 = 1 and F2 = 1.
///
/// For this problem we shall be interested in values of x for which AF(x) is a
/// positive integer.
///
/// We shall call AF(x) a golden nugget if x is rational, because they become
/// increasingly rarer; for example, the 10th golden nugget is 74049690.
///
/// Find the 15th golden nugget.
fn main() {
    // A_F (x) is simply the generating function for the Fibonacci sequence, and
    // its value is given by:
    // A_F (x) = x / (1 - x - x^2)
    // By the quadratic formula, replacing A_F (x) with a, we get that:
    // x = (sqrt(5 a^2 + 2a + 1) - a - 1) / (2 * a)
    // Thus, x will be rational if the discriminant is a perfect square.
    // Interestingly, the solutions are given by F(2n) * F(2n + 1), starting at
    // n = 1.
    let answer: u64 = Fibonacci::<u64>::new()
        .skip(2 * TARGET - 1)
        .take(2)
        .product();

    println!("The answer is: {}", answer);
}
