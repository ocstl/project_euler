use num::integer::Integer;

const MAX_D: u64 = 12_000;
const SMALLEST: u64 = 3;
const LARGEST: u64 = 2;

/// How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for
/// d ≤ 12,000?
fn main() {
    // Start with the denominator of the mediant of the two fractions, then count the proper
    // fractions.
    let answer = (SMALLEST + LARGEST..=MAX_D)
        .flat_map(|d| ((d / SMALLEST)..(d / LARGEST)).filter(move |n| n.gcd(&d) == 1))
        .count();

    println!("The answer is: {}", answer);
}
