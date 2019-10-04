use radixal::IntoDigits;

const INPUT: usize = 10_000_000;
const TARGET: usize = 89;

/// A number chain is created by continuously adding the square of the digits in a number to form
/// a new number until it has been seen before.
///
/// For example,
///
/// 44 → 32 → 13 → 10 → 1 → 1
/// 85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
///
/// Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most
/// amazing is that EVERY starting number will eventually arrive at 1 or 89.
///
/// How many starting numbers below ten million will arrive at 89?
fn main() {
    // Store previously visited numbers to speed up the process.
    let mut endpoints: Box<[Option<usize>]> = vec![None; INPUT].into_boxed_slice();
    endpoints[1] = Some(1);
    endpoints[89] = Some(89);

    for n in 2..INPUT {
        let chain: Vec<usize> = std::iter::successors(Some(n), |&m| {
            if endpoints[m].is_none() {
                Some(square_digit_chain(m))
            } else {
                None
            }
        })
        .collect();
        let endpoint = endpoints[*chain.last().unwrap()];
        for m in chain {
            endpoints[m] = endpoint;
        }
    }

    let answer = endpoints
        .iter()
        .filter(|&&endpoint| endpoint == Some(TARGET))
        .count();
    println!("The answer is: {}", answer);
}

fn square_digit_chain(n: usize) -> usize {
    n.into_decimal_digits().map(|digit| digit * digit).sum()
}
