use primal::{Primes, Sieve};

const MAX_VALUE: usize = 1000;
const MAX_VALUE_ISIZE: isize = 1000;

/// Find the product of the coefficients, a and b, for the quadratic expression that produces the
/// maximum number of primes for consecutive values of n, starting with n=0.
/// The formula: n^2 + a*n + b, where |a| < 1000 and |b| <= 1000.
fn main() {
    // Maximum possible prime produced by the formula.
    let sieve = Sieve::new(2 * MAX_VALUE * MAX_VALUE + MAX_VALUE);
    let check_prime = |x: isize| x > 0 && sieve.is_prime(x as usize);

    // Since we start at 0, b has to be prime.
    // To get more than n = 0, 1 + a + b >= 2, so a <= 1 - b.
    let iter_b = Primes::all().take_while(|&x| x <= MAX_VALUE);
    let iter_a = |b: isize| ((1 - b)..MAX_VALUE_ISIZE);

    let answer = iter_b
        .flat_map(|b| iter_a(b as isize).map(move |a| (a, b as isize)))
        .map(|(a, b)| {
            let f = |x| x * x + a * x + b;
            let length = (0..).take_while(|&x| check_prime(f(x))).count();
            (length, a * b)
        })
        .max()
        .unwrap();

    println!("Answer: {:?}", answer);
}
