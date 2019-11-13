use std::cmp::Ordering;
use std::collections::BinaryHeap;

const MAX_K: u32 = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdditionChain(usize, Vec<u32>);

impl AdditionChain {
    fn new() -> Self {
        // Don't count the initial value as a multiplication.
        AdditionChain(0, vec![1])
    }

    fn last(&self) -> u32 {
        *self.1.last().unwrap()
    }

    fn next(self) -> Vec<AdditionChain> {
        let l = self.0;
        let v = self.1;
        let last = *v.last().unwrap();

        v.iter()
            .map(|&n| {
                let mut new_chain = v.clone();
                new_chain.push(last + n);
                AdditionChain(l + 1, new_chain)
            })
            .collect()
    }
}

// Since `BinaryHeap` is a max-heap, reverse the ordering to get the smallest length first.
impl PartialOrd for AdditionChain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0).reverse().then(self.1.cmp(&other.1)))
    }
}

impl Ord for AdditionChain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse().then(self.1.cmp(&other.1))
    }
}

fn shortest_addition_chain(n: u32) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(AdditionChain::new());

    while let Some(chain) = queue.pop() {
        if chain.last() == n {
            return chain.0;
        } else if chain.last() < n {
            queue.extend(chain.next())
        }
    }

    unreachable!();
}

/// The most naive way of computing n15 requires fourteen multiplications.
/// But using a "binary" method you can compute it in six multiplications.
/// However it is yet possible to compute it in only five multiplications.
///
/// We shall define m(k) to be the minimum number of multiplications to compute nk; for example m
/// (15) = 5.
///
/// For 1 ≤ k ≤ 200, find ∑ m(k).
fn main() {
    let answer: usize = (1..=MAX_K).map(shortest_addition_chain).sum();

    println!("The answer is: {}", answer);
}
