const INPUT: u128 = 100;

fn combinations(n: u128, choose: u128) -> u128 {
    ((n - choose + 1)..=n).product::<u128>() / (1..=choose).product::<u128>()
}

/// Working from left-to-right if no digit is exceeded by the digit to its left it is called an
/// increasing number; for example, 134468.
///
/// Similarly if no digit is exceeded by the digit to its right it is called a decreasing number;
/// for example, 66420.
///
/// We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number;
/// for example, 155349.
///
/// As n increases, the proportion of bouncy numbers below n increases such that there are only
/// 12951 numbers below one-million that are not bouncy and only 277032 non-bouncy numbers below
/// 10^10.
///
/// How many numbers below a googol (10^100) are not bouncy?
fn main() {
    // There are a lot of these, so let's use combinations. We can think of these numbers as a
    // sequence of n states, with k transitions, for a sequence length of n + k. For example,
    // with 3 states (digits), we can pepper up to 9 transitions for ascending numbers:
    //
    // 012: STSTSTTTTTTT
    // 022: STTSSTTTTTTT
    // 489: TTTTSTTTTSTS
    //
    // Note that some of the transitions can be put at the end, thus having no "effect", but
    // always yielding a 3 + 9 == 12 element sequence.

    // Ascending: 100 hundred digits, but put up to 9 transitions in there (0 -> 1, 2 -> 3, etc.).
    let ascending = combinations(INPUT + 9, 9);

    // Descending: 100 hundred digits, but put up to 9 transitions in there (1 -> 0, 3 -> 2, etc
    // .), plus 1 for the 0 -> x transition to reflect shorter numbers.
    let descending = combinations(INPUT + 10, 10);

    // Remove duplicates. "888" will be present in both, so we need to remove all of those. So 1
    // of 10 digits, with 1 transition (0 -> x) over 100 positions.
    let duplicates = 10 * combinations(INPUT, 1);

    // Don't forget 0! We don't want it, but it's still in both ascending and descending.
    let zeroes = 2;

    let answer = ascending + descending - duplicates - zeroes;
    println!("The answer is: {}", answer);
}
