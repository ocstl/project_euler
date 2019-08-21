use project_euler::totient;

const INPUT: usize = 1_000_000;

/// Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
/// number of numbers less than n which are relatively prime to n.
///
/// Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
fn main() {
    let f = totient::totient_function(INPUT);

    let answer = (2..=INPUT)
        .map(|n| (n as f64 / f(n).unwrap() as f64, n))
        .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}
