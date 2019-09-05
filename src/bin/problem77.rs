use primal::Primes;

const INPUT: usize = 5000;

/// It is possible to write ten as the sum of primes in exactly five different ways:
/// * 7 + 3
/// * 5 + 5
/// * 5 + 3 + 2
/// * 3 + 3 + 2 + 2
/// * 2 + 2 + 2 + 2 + 2
///
/// What is the first value which can be written as the sum of primes in over five thousand
/// different ways?
fn main() {
    let possible_values =
        |n: usize| -> Vec<usize> { Primes::all().take_while(|&p| p <= n).collect() };

    let answer = (2..)
        .find(|&n| nbr_partitions(n, &possible_values(n)) >= INPUT)
        .expect("Didn't find any.");

    println!("The answer is: {}", answer);
}

// Reuse the code from problem 76.
fn nbr_partitions(n: usize, possible_values: &[usize]) -> usize {
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
