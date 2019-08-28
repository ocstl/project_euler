const MAX_D: u32 = 1_000_000;
const THREE_SEVENTHS: Ratio = (3, 7);

type Ratio = (u32, u32);

/// By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size,
/// find the numerator of the fraction immediately to the left of 3/7.
fn main() {
    // The listed proper fractions for d <= 8 put 2/5 left of 3/7. Sum the numerators and
    // denominators to generate the mediants of the fraction between the two.
    let ratios = std::iter::successors(Some((2, 5)), |ratio| {
        Some((ratio.0 + THREE_SEVENTHS.0, ratio.1 + THREE_SEVENTHS.1))
    });

    let answer = ratios.take_while(|ratio| ratio.1 < MAX_D).last().unwrap();

    println!("The answer is: {}", answer.0);
}
