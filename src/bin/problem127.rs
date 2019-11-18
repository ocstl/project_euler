use num::integer::Integer;
use primal::Sieve;

const INPUT: usize = 120_000;

/// The radical of n, rad(n), is the product of distinct prime factors of n.
/// For example, 504 = 2^3 × 3^2 × 7, so rad(504) = 2 × 3 × 7 = 42.
///
/// We shall define the triplet of positive integers (a, b, c) to be an abc-hit if:
///
/// 1. GCD(a, b) = GCD(a, c) = GCD(b, c) = 1
/// 2. a < b
/// 3. a + b = c
/// 4. rad(abc) < c
///
/// For example, (5, 27, 32) is an abc-hit, because:
///
/// 1. GCD(5, 27) = GCD(5, 32) = GCD(27, 32) = 1
/// 2. 5 < 27
/// 3. 5 + 27 = 32
/// 4. rad(4320) = 30 < 32
///
/// It turns out that abc-hits are quite rare and there are only thirty-one
/// abc-hits for c < 1000, with ∑c = 12523.
///
/// Find ∑c for c < 120000.
fn main() {
    // GCD(a, b) == 1 only means that a and b do not share a common prime factor.
    // Alternatively, GCD(rad(a), rad(b)) == 1 as well, so we don't need to
    // carry the prime factors around (and we assume that rad(1) = 1).
    let sieve = Sieve::new(INPUT);
    let mut radicals = Vec::with_capacity(INPUT);
    radicals.push(0);
    radicals.push(1);
    for n in 2..INPUT {
        let radical = sieve
            .factor(n)
            .unwrap()
            .into_iter()
            .map(|(p, _)| p)
            .product();

        radicals.push(radical);
    }

    // Since a, b and c do not share a common prime factor (GCD == 1),
    // rad(abc) = rad(a) * rad(b) * rad(c). Also, if the radicals of a and b do
    // not share a prime factor, then neither will a and c or b and c.
    let answer: usize = (1..(INPUT / 2))
        .flat_map(|a| ((a + 1)..(INPUT - a)).map(move |b| (a, b, a + b)))
        .filter_map(|(a, b, c)| {
            if ((radicals[a] * radicals[b] * radicals[c]) < c) && radicals[a].gcd(&radicals[b]) == 1
            {
                Some(c)
            } else {
                None
            }
        })
        .sum();

    println!("The answer is: {}", answer);
}
