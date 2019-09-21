use counter::Counter;
use project_euler::monopoly;

const BURN_IN: usize = 100_000;
// The ordering is very unstable around the third member, possibly because the chance and
// community chest implementations are hacky.
const TURNS: usize = 20_000_000;

/// By starting at GO and numbering the squares sequentially from 00 to 39 we can concatenate
/// these two-digit numbers to produce strings that correspond with sets of squares.
///
/// Statistically it can be shown that the three most popular squares, in order, are JAIL (6.24%)
/// = Square 10, E3 (3.18%) = Square 24, and GO (3.09%) = Square 00. So these three most popular
/// squares can be listed with the six-digit modal string: 102400.
///
/// If, instead of using two 6-sided dice, two 4-sided dice are used, find the six-digit modal
/// string.
fn main() {
    let frequencies: Counter<usize, usize> = monopoly::Player::new()
        .skip(BURN_IN)
        .take(TURNS)
        .map(|player| player.location())
        .collect();

    let answer: String = frequencies.most_common_ordered()[0..3]
        .iter()
        .map(|&(square, _)| format!("{:02}", square))
        .collect();

    println!("{}", answer);
}
