const MIN_A: u64 = 3;
const MAX_A: u64 = 1000;

/// Let r be the remainder when (a−1)^n + (a+1)^n is divided by a^2.
///
/// For example, if a = 7 and n = 3, then r = 42: 6^3 + 8^3 = 728 ≡ 42 mod 49. And as n varies, so
/// too will r, but for a = 7 it turns out that r_max = 42.
///
/// For 3 ≤ a ≤ 1000, find ∑ r_max.
fn main() {
    // (a−1)^n + (a+1)^n mod a^2 yields 2 if n is even, 2na mod a^2 if n is odd. So, the smallest
    // n that yields r_max is strictly less than a / 2, so we can take the floor of (a - 1) / 2.
    let answer: u64 = (MIN_A..=MAX_A).map(|a| (a * ((a - 1) >> 1)) << 1).sum();

    println!("The answer is: {}", answer);
}
