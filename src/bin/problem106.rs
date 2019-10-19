const INPUT: usize = 12;

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}

fn combinations(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

/// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set
/// if for any two non-empty disjoint subsets, B and C, the following properties are true:
///
/// S(B) ≠ S(C); that is, sums of subsets cannot be equal.
/// If B contains more elements than C then S(B) > S(C).
///
/// For this problem we shall assume that a given set contains n strictly increasing elements and
/// it already satisfies the second rule.
///
/// Surprisingly, out of the 25 possible subset pairs that can be obtained from a set for which n
/// = 4, only 1 of these pairs need to be tested for equality (first rule). Similarly, when n = 7, only 70 out of the 966 subset pairs need to be tested.
///
/// For n = 12, how many of the 261625 subset pairs that can be obtained need to be tested for
/// equality?
///
/// NOTE: This problem is related to Problem 103 and Problem 105.
fn main() {
    // Rule 2 has already dealt with subset pairs with an unequal number of elements, which leaves
    // only comparisons for subsets having the same number of elements. Since all the elements
    // are in increasing order, we can eliminate all pairs of subsets of length 1.
    // We can easily find the number of combinations of 4, 6, 8, ... elements in a set of size n,
    // but half of those end up being the same comparison, so: n! / (2 * k! (n-k)!).
    // If we consider the first half of the elements as opening brackets and the other half as
    // closing brackets, ([ [ ] ]), we are only interested in unbalanced strings of brackets. In
    // other words, from the possible combinations above, we do not need to compare Dyck words.
    let answer: usize = (2..=(INPUT / 2))
        .map(|k| {
            let all_pairs = combinations(INPUT, k << 1);
            all_pairs * combinations(k << 1, k) * (k - 1) / ((k + 1) << 1)
        })
        .sum();

    println!("The answer is: {}", answer);
}
