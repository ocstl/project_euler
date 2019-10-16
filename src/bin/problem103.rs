use itertools::Itertools;
use std::collections::HashSet;

const SET_SIZE: usize = 7;

/// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set
/// if for any two non-empty disjoint subsets, B and C, the following properties are true:
///
/// 1. S(B) ≠ S(C); that is, sums of subsets cannot be equal.
/// 2.0 If B contains more elements than C then S(B) > S(C).
///
/// If S(A) is minimised for a given n, we shall call it an optimum special sum set. The first
/// five optimum special sum sets are given below.
///
/// n = 1: {1}
/// n = 2: {1, 2}
/// n = 3: {2, 3, 4}
/// n = 4: {3, 5, 6, 7}
/// n = 5: {6, 9, 11, 12, 13}
///
/// Given that A is an optimum special sum set for n = 7, find its set string.
fn main() {
    // Magic number 45. Very hacky.
    let answer = generate_sets(SET_SIZE, 45)
        .filter(|set| rule1(set) && rule2(set))
        .min_by_key(|set| set.iter().sum::<usize>())
        .unwrap();

    let answer: String = answer
        .into_iter()
        .map(|number| number.to_string())
        .collect();

    println!("The answer is: {:?}", answer);
}

// This is not very efficient.
fn generate_sets(nbr_elements: usize, max_element: usize) -> impl Iterator<Item = Vec<usize>> {
    (1..=max_element).combinations(nbr_elements)
}

fn rule1(set: &[usize]) -> bool {
    let mut sums: HashSet<usize> = HashSet::new();

    (1..=set.len())
        .flat_map(|l| set.iter().combinations(l))
        .all(|window| sums.insert(window.into_iter().sum()))
}

fn rule2(set: &[usize]) -> bool {
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
