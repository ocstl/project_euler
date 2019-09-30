use std::collections::HashSet;

const INPUT: usize = 12_000;

/// A natural number, N, that can be written as the sum and product of a given set of at least
/// two natural numbers, {a1, a2, ... , ak} is called a product-sum number: N = a1 + a2 + ... +
/// ak = a1 × a2 × ... × ak.
///
/// For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.
///
/// For a given set of size, k, we shall call the smallest N with this property a minimal
/// product-sum number.
///
/// What is the sum of all the minimal product-sum numbers for 2 <= k <= 12_000?
fn main() {
    // Assuming that all a's are 1, we get a minimum sum of k, which gives a lower bound.
    // Also, N has to be composite, except when k = 1. a_k has to be different from 1, which
    // implies that a_k-1 is greater than 1, otherwise the sum would be greater than the product.
    // Thus, we have an upper bound of of {..., 2, k}; we can also refine the lower bound since k
    // >= 2; +1 for a_k-1 and +1 for a_k.
    // k + 2 <= N <= 2k.
    // We could go even further in this: the sum will be minimized when all a's are the same;
    // thus, an even stricter bound is obtained by using k * (k + 2)^(1 / k), but the gain is
    // limited for the work: at k = 12_000, we get a bound of 12_010 (rounded up), rather than
    // 12_002.
    let find_n = |k: usize| {
        ((k + 2)..=(2 * k))
            .find(|&n| {
                factorisations(n)
                    .iter()
                    .any(|v| v.iter().sum::<usize>() + k - v.len() == n)
            })
            .unwrap()
    };

    let answer: usize = (2..=INPUT)
        .map(find_n)
        .collect::<HashSet<usize>>()
        .into_iter()
        .sum();
    println!("The answer is: {}", answer);
}

// There is a better way, but this'll do. Saving only the sums would work as well.
fn factorisations(n: usize) -> HashSet<Vec<usize>> {
    let mut results = HashSet::new();

    for factor in 2..=(n / 2) {
        if n % factor == 0 {
            results.insert(vec![factor, n / factor]);
            let sub_factorisations = factorisations(n / factor);
            for mut sub in sub_factorisations.into_iter() {
                sub.push(factor);
                sub.sort();
                results.insert(sub);
            }
        }
    }

    results
}
