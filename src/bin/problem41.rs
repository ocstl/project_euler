use primal::Sieve;
use project_euler::unsigned::UnsignedInteger;

/* Since any 9-digital and 8-digital number is divisible by 3 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 ==
 * 36 % 3 == 0, and 36 + 9 % 3 == 0), we need only explore up to 7 digits numbers. */
const INPUT: usize = 7_654_321;
const BASE: usize = 10;

fn is_pandigital(n: usize) -> bool {
    let digits = n.to_radix_le(BASE);
    (1..=digits.len()).all(|x| digits.contains(&x))
}

fn main() {
    let answer = Sieve::new(INPUT)
        .primes_from(0)
        .take_while(|&x| x <= INPUT)
        .filter(|&x| is_pandigital(x))
        .max()
        .unwrap();

    println!("Answer: {}", answer);
}
