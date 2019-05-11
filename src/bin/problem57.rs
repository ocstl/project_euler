use num::BigUint;

const INPUT: usize = 1000;

fn main() {
    // The square root of 2 has quotients equal to 2, so we can generate the numerators and
    // denominators of the convergents by iteration.
    let quotient = 2u8;
    let mut numerators = vec![BigUint::from(1u8), BigUint::from(3u8)];
    let numerator = core::iter::from_fn(|| {
        let mut it = numerators.iter().rev();
        let n = quotient * it.next().unwrap() + it.next().unwrap();
        numerators.push(n.clone());
        Some(n)
    });

    let mut denominators = vec![BigUint::from(1u8), BigUint::from(2u8)];
    let denominator = core::iter::from_fn(|| {
        let mut it = denominators.iter().rev();
        let n = quotient * it.next().unwrap() + it.next().unwrap();
        denominators.push(n.clone());
        Some(n)
    });

    // Find the number of convergents where the numerator has more digits (decimal) than the
    // denominator.
    let answer = numerator
        .zip(denominator)
        .take(INPUT)
        .filter(|(num, den)| num.to_radix_le(10).len() > den.to_radix_le(10).len())
        .count();

    println!("Answer: {}", answer);
}
