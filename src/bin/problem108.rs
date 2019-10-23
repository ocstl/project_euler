use primal::Sieve;

const INPUT: usize = 1000;

/// In the following equation x, y, and n are positive integers.
///
/// 1 / x + 1 / y = 1 / n
///
/// For n = 4 there are exactly three distinct solutions:
///
/// 1 / 5 + 1 / 20 = 1 / 4
/// 1 / 6 + 1 / 12 = 1 / 4
/// 1 / 8 + 1 / 8 = 1 / 4
///
/// What is the least value of n for which the number of distinct solutions exceeds one-thousand?
fn main() {
    // Letting x = n + a, y = n + b, we get 1 / (n + a) + 1 / (n + b) = 1 / n. After some
    // rearranging, we get n^2 = ab. Thus, if we know the number of factors of n^2 (say k), we know
    // how many solutions we have, (k + 1) / 2, since a and b are interchangeable (+ 1 since we
    // might have an odd number of factors). Now, since n^2 can get large, it is easier to
    // factorize n and get the number of factors of n^2.
    let sieve = Sieve::new(1_000_000);

    let nbr_factors_square = |n: usize| -> usize {
        sieve
            .factor(n)
            .unwrap()
            .iter()
            .map(|v| (v.1 << 1) + 1)
            .product::<usize>()
    };

    let answer = (1..)
        .find(|&n| ((nbr_factors_square(n) + 1) >> 1) > INPUT)
        .unwrap();

    println!("The answer is: {}", answer);
}
