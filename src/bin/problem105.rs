use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "inputs/p105_sets.txt";

/// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set
/// if for any two non-empty disjoint subsets, B and C, the following properties are true:
///
/// S(B) ≠ S(C); that is, sums of subsets cannot be equal.
/// If B contains more elements than C then S(B) > S(C).
///
/// For example, {81, 88, 75, 42, 87, 84, 86, 65} is not a special sum set because 65 + 87 + 88 =
/// 75 + 81 + 84, whereas {157, 150, 164, 119, 79, 159, 161, 139, 158} satisfies both rules for
/// all possible subset pair combinations and S(A) = 1286.
///
/// Using sets.txt, a 4K text file with one-hundred sets containing seven to twelve elements (the
/// two examples given above are the first two sets in the file), identify all the special sum
/// sets, A1, A2, ..., Ak, and find the value of S(A1) + S(A2) + ... + S(Ak).
///
/// NOTE: This problem is related to Problem 103 and Problem 106.
fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();
    let answer: u64 = input
        .lines()
        .filter_map(|line| {
            let set: Vec<u64> = line
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .sorted()
                .collect();
            if rule1(&set) && rule2(&set) {
                Some(set.into_iter().sum::<u64>())
            } else {
                None
            }
        })
        .sum();

    println!("The answer is: {}", answer);
}

fn rule1(set: &[u64]) -> bool {
    let mut sums: HashSet<u64> = HashSet::new();

    // `HashSet::insert` returns `false` if the element was already present.
    (1..set.len())
        .flat_map(|l| set.iter().combinations(l))
        .all(|window| sums.insert(window.into_iter().sum()))
}

// Make sure that the values are ordered, so that we need only compare once.
fn rule2(set: &[u64]) -> bool {
    let mut iter = set.iter();
    let mut sum_start = *iter.next().unwrap();
    let mut sum_end = 0;

    while let (Some(a), Some(b)) = (iter.next(), iter.next_back()) {
        sum_start += *a;
        sum_end += *b;

        if sum_end > sum_start {
            return false;
        }
    }

    true
}
