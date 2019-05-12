use primal::is_prime;

fn spiral_corners(side_length: u64) -> Vec<u64> {
    // Generate all four corners for a spiral whose side is `n`.
    match side_length {
        0 => Vec::new(),
        1 => vec![1],
        n if n % 2 == 1 => {
            let step = n - 1;
            let end = n * n;
            vec![end - 3 * step, end - 2 * step, end - step, end]
        }
        _ => panic!("`side_length` should be odd."),
    }
}

fn main() {
    let answer = (3..)
        .step_by(2)
        .scan((0, 1), |acc, side_length| {
            dbg!(side_length);
            acc.0 += spiral_corners(side_length)
                .iter()
                .filter(|&&n| is_prime(n))
                .count();
            acc.1 += 4;
            Some(*acc)
        })
        .find(|&(a, b)| 10 * a < b)
        .map(|(_, corners)| 1 + corners / 2)
        .unwrap();

    println!("Answer: {}", answer);
}
