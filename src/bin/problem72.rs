use project_euler::totient::totient_function;

const MAX_D: usize = 1_000_000;

/// How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?
fn main() {
    // Since the numerator and denominator of a proper fraction are coprime, we can use Euler's
    // totient function to count the number of proper fractions with a given denominator.
    let totient = totient_function(MAX_D);
    let answer: usize = (2..=MAX_D).map(totient).sum::<Result<usize, _>>().unwrap();

    println!("The answer is: {}", answer);
}
