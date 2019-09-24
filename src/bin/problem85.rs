use num::integer::Roots;
use std::iter::once;

const INPUT: i64 = 2_000_000;

/// By counting carefully it can be seen that a rectangular grid measuring 3 by 2 contains
/// eighteen rectangles.
///
/// Although there exists no rectangular grid that contains exactly two million rectangles, find
/// the area of the grid with the nearest solution.
fn main() {
    let max_line_length = smallest_line_length(INPUT) + 1;

    let answer = (1..=(max_line_length / 2))
        .flat_map(|m| {
            let m_rectangles = nbr_rectangles(m);
            let n = smallest_line_length(INPUT / m_rectangles);

            // Construct a (nbr_rectangles, area) tuple to find the closest result to INPUT.
            once((m_rectangles * nbr_rectangles(n), m * n))
                .chain(once((m_rectangles * nbr_rectangles(n + 1), m * (n + 1))))
        })
        .min_by_key(|(nbr, _)| (INPUT - nbr).abs())
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}

/// A line of n squares contains n * (n + 1) / 2 rectangles.
fn nbr_rectangles(n: i64) -> i64 {
    n * (n + 1) / 2
}

/// Find the smallest to fit under (or equal) to the required number of rectangles.
fn smallest_line_length(n: i64) -> i64 {
    (-1 + (1 + 8 * n).sqrt()) / 2
}
