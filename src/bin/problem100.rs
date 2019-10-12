const INPUT: u64 = 1_000_000_000_000;

/// If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red
/// discs, and two discs were taken at random, it can be seen that the probability of taking two
/// blue discs, P(BB) = (15/21)×(14/20) = 1/2.
///
/// The next such arrangement, for which there is exactly 50% chance of taking two blue discs at
/// random, is a box containing eighty-five blue discs and thirty-five red discs.
///
/// By finding the first arrangement to contain over 10^12 = 1,000,000,000,000 discs in total,
/// determine the number of blue discs that the box would contain.
fn main() {
    // This can be dealt with as a quadratic Diophantine equation.
    let mut solutions = std::iter::successors(Some((15, 21)), |s| {
        Some((3 * s.0 + 2 * s.1 - 2, 4 * s.0 + 3 * s.1 - 3))
    });

    let answer = solutions.find(|s| s.1 > INPUT).unwrap().0;

    println!("The answer is: {}", answer);
}
