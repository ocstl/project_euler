const INPUT: u64 = 100;

/// It is possible to write five as a sum in exactly six different ways:
/// * 4 + 1
/// * 3 + 2
/// * 3 + 1 + 1
/// * 2 + 2 + 1
/// * 2 + 1 + 1 + 1
/// * 1 + 1 + 1 + 1 + 1
///
/// How many different ways can one hundred be written as a sum of at least two positive integers?
fn main() {
    let possible_values = (1..INPUT).rev().collect::<Vec<u64>>();
    let answer = nbr_partitions(INPUT, &possible_values);

    println!("The answer is: {}", answer);
}

// This is very slow. Should explore using Euler's generating function.
fn nbr_partitions(n: u64, possible_values: &[u64]) -> usize {
    /* Return early when impossible. */
    if n != 0 && possible_values.is_empty() {
        0
    } else {
        let value = possible_values[0];
        let max_nbr = n / value;

        (0..=max_nbr)
            .map(|x| {
                let new_n = n - x * value;
                if new_n == 0 {
                    1
                } else {
                    nbr_partitions(new_n, &possible_values[1..])
                }
            })
            .sum()
    }
}
