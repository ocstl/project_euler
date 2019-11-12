const TURNS: usize = 15;

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// A bag contains one red disc and one blue disc. In a game of chance a player takes a disc at
/// random and its colour is noted. After each turn the disc is returned to the bag, an extra red
/// disc is added, and another disc is taken at random.
///
/// The player pays £1 to play and wins if they have taken more blue discs than red discs at the
/// end of the game.
///
/// If the game is played for four turns, the probability of a player winning is exactly 11/120,
/// and so the maximum prize fund the banker should allocate for winning in this game would be
/// £10 before they would expect to incur a loss. Note that any payout will be a whole number of
/// pounds and also includes the original £1 paid to play the game, so in the example given the
/// player actually wins £9.
///
/// Find the maximum prize fund that should be allocated to a single game in which fifteen turns
/// are played.
fn main() {
    // The probability of getting a blue disc at each turn is 2 + n, with n being the turn (0, 1,
    // ...).
    let probabilities: Vec<usize> = (2..).take(TURNS).collect();

    // We need at least 1 + TURNS / 2 blue to win. Let a set bit represent a blue, so we only
    // count the number with enough set bits.
    let min_blues = ((TURNS >> 1) + 1) as u32;
    let win: usize = (0..(1 << TURNS))
        .filter(|draw: &usize| draw.count_ones() >= min_blues)
        .map(|mut draw| {
            probabilities
                .iter()
                .map(|&discs| {
                    let p = if draw & 1 == 1 { 1 } else { discs - 1 };
                    draw >>= 1;
                    p
                })
                .product::<usize>()
        })
        .sum();

    // The total number of possible draws is (TURNS + 1)!.
    let answer = factorial(TURNS + 1) / win;
    println!("The answer is: {}", answer);
}
