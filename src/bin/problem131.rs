use primal::is_prime;

const INPUT: u64 = 1_000_000;

/// There are some prime values, p, for which there exists a positive integer,
/// n, such that the expression n^3 + n^2 * p is a perfect cube.
///
/// For example, when p = 19, 8^3 + 8^2 * 19 = 123.
///
/// What is perhaps most surprising is that for each prime with this property
/// the value of n is unique, and there are only four such primes below
/// one-hundred.
///
/// How many primes below one million have this remarkable property?
fn main() {
    // Since n^2 (n + p) is a perfect cube, by the unique factorization theorem,
    // both n^2 and (n + p) are perfect cubes, since p is prime. Interestingly,
    // since n^2 is also a square, the unique factorization theorem also gives
    // us that n is a perfect cube: if any prime factor of n has an exponent not
    // divisible by 3, then multiplying by 2 will not make it divisible by 3,
    // thus not a cube.
    // So, if n = a^3 and n + p = b^3, p = b^3 - a^3.
    // We could iterate over all the pairs of cubes, but, by simple
    // factorization:
    // p = (b - a) * (b^2 + ba + a^2)
    // Since p is prime, one of these must be 1. Clearly, b - a is the smaller
    // one, so b = a + 1, and:
    // p = b^2 + ba + a^2
    // p = (a + 1)^2 + (a + 1) * a + a^2
    // p = 3a^2 + 3a + 1
    let answer: usize = (1..)
        .filter_map(|a| Some(3 * (a + 1) * a + 1).filter(|&p| is_prime(p)))
        .take_while(|&p| p < INPUT)
        .count();

    println!("The answer is: {}", answer);
}
