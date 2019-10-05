use itertools::Itertools;
use permutohedron::Heap;
use std::collections::BTreeSet;
use std::iter::once;

const INPUT: usize = 4;

/// By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and making use of the
/// four arithmetic operations (+, −, *, /) and brackets/parentheses, it is possible to form
/// different positive integer targets.
///
/// For example,
///
/// 8 = (4 * (1 + 3)) / 2
/// 14 = 4 * (3 + 1 / 2)
/// 19 = 4 * (2 + 3) − 1
/// 36 = 3 * 4 * (2 + 1)
///
/// Note that concatenations of the digits, like 12 + 34, are not allowed.
///
/// Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different target numbers of
/// which 36 is the maximum, and each of the numbers 1 to 28 can be obtained before encountering
/// the first non-expressible number.
///
/// Find the set of four distinct digits, a < b < c < d, for which the longest set of consecutive
/// positive integers, 1 to n, can be obtained, giving your answer as a string: abcd.
fn main() {
    let answer = (0_i64..=9)
        .combinations(INPUT)
        .max_by_key(|combination| {
            let targets = all_permutations(combination);
            (1..).find(|target| !targets.contains(target))
        })
        .unwrap();

    println!("The answer is: {:?}", answer);
}

fn all_permutations(number: &[i64]) -> BTreeSet<i64> {
    let mut n = [1, 2, 3, 4];
    n.copy_from_slice(&number[0..INPUT]);
    let heap = Heap::new(&mut n);

    heap.flat_map(|order| all_targets(&order)).collect()
}

fn all_targets(numbers: &[i64; 4]) -> impl Iterator<Item = i64> {
    let &[a, b, c, d] = numbers;
    // Consecutive: ((a + b) + c) + d
    let consecutive = binary_ops(a, b)
        .flat_map(move |ab| binary_ops(ab, c))
        .flat_map(move |abc| binary_ops(abc, d));

    // Pairs: (a + b) + (c + d)
    let abs = binary_ops(a, b);
    let cds = binary_ops(c, d);
    let pairs = abs
        .cartesian_product(cds)
        .flat_map(|(ab, cd)| binary_ops(ab, cd));

    consecutive.chain(pairs).filter(|n| n.is_positive())
}

fn binary_ops(a: i64, b: i64) -> impl Iterator<Item = i64> + Clone {
    once(a + b)
        .chain(once(a - b))
        .chain(once(a * b))
        // Only retain divisions without remainders.
        .chain(
            if b != 0 && a % b == 0 {
                Some(a / b)
            } else {
                None
            }
            .into_iter(),
        )
}
