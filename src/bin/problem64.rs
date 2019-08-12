use project_euler::continued_fractions::square_root::SquareRoot;

const N: u32 = 10_000;

/// All square roots are periodic when written as continued fractions. For conciseness, we use
/// the notation √23 = [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats indefinitely.
/// How many continued fractions for N <= 10000 have an odd period?
fn main() {
    let answer = (1..=N)
        .filter(|&n| SquareRoot::new(n).period_length() % 2 == 1)
        .count();

    println!("The answer is: {}", answer);
}
