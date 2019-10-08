use project_euler::aliquot_sum_fn;
use std::collections::HashSet;

const INPUT: usize = 1_000_000;

aliquot_sum_fn!(INPUT);

fn build_chain(number: usize) -> Option<HashSet<usize>> {
    let mut chain = HashSet::new();
    let mut n = number;

    while chain.insert(n) {
        n = aliquot_sum(n);
        if n > INPUT {
            return None;
        }
    }

    // Return the chain if it returns to its starting point, `None` otherwise.
    if n == number {
        Some(chain)
    } else {
        None
    }
}

/// The proper divisors of a number are all the divisors excluding the number itself. For
/// example, the proper divisors of 28 are 1, 2, 4, 7, and 14. As the sum of these divisors is
/// equal to 28, we call it a perfect number.
///
/// Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors
/// of 284 is 220, forming a chain of two numbers. For this reason, 220 and 284 are called an
/// amicable pair.
///
/// Perhaps less well known are longer chains. For example, starting with 12496, we form a chain
/// of five numbers:
///
/// 12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)
///
/// Since this chain returns to its starting point, it is called an amicable chain.
///
/// Find the smallest member of the longest amicable chain with no element exceeding one million.
fn main() {
    let mut chains = vec![None; INPUT + 1].into_boxed_slice();

    for x in 0..=INPUT {
        if chains[x].is_some() {
            continue;
        }

        if let Some(chain) = build_chain(x) {
            let l = chain.len();
            chain.iter().for_each(|&idx| chains[idx] = Some(l));
        }
    }

    let longest_chain = chains.iter().max().unwrap();
    let answer = chains
        .iter()
        .enumerate()
        .find(|&(_, l)| l == longest_chain)
        .unwrap()
        .0;

    println!("The answer is: {}", answer);
}
