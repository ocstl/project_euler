const INPUT: usize = 100;
const MIN_COMBINATIONS: usize = 1_000_000;

fn number_of_combinations(n: usize) -> impl Iterator<Item = usize> {
    let mut r = 0;
    let mut nbr = 1;

    std::iter::from_fn(move || {
        r += 1;
        if r < n {
            nbr = nbr * (n - r + 1) / r;
            Some(nbr)
        } else {
            None
        }
    })
}

fn main() {
    let answer: usize = (1..=INPUT)
        .map(|n| {
            // Find the smallest r that generates more than `MIN_COMBINATIONS` combinations. Since
            // enumerate starts at 0, offset by 1. The valid numbers are between r and n - r
            // inclusive.
            if let Some((r, _)) = number_of_combinations(n)
                .enumerate()
                .find(|&(_, x)| x >= MIN_COMBINATIONS)
            {
                n - (r + 1) - r
            } else {
                0
            }
        })
        .sum();

    println!("Answer: {}", answer);
}
