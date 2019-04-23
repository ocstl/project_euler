const INPUT: [usize; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];

/* Outputs the digits in order from least significant to most significant. Use .iter().rev() to
 * reverse order. */
fn to_digits(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![0]
    } else {
        let mut n = n;
        core::iter::from_fn(move || {
            if n > 0 {
                let rem = n % 10;
                n /= 10;
                Some(rem)
            } else {
                None
            }
        })
        .collect()
    }
}

fn main() {
    /* Iterator over the digits. Since FromFn doesn't implement DoubleEndedIterator, we use a
     * workaround (Vec). */
    let digits = (0..).flat_map(|n| to_digits(n).into_iter().rev());

    let answer: usize = digits
        .enumerate()
        .filter_map(|(idx, n)| if INPUT.contains(&idx) { Some(n) } else { None })
        /* Watch out for the infinite iterator! */
        .take(INPUT.len())
        .product();

    println!("Answer: {}", answer);
}
