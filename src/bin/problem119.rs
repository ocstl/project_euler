use radixal::IntoDigits;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

const INPUT: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PowerSum {
    number: u64,
    digits_sum: u64,
    first_seen: bool,
}

impl PowerSum {
    fn new(digits_sum: u64) -> Self {
        PowerSum {
            number: digits_sum * digits_sum,
            digits_sum,
            first_seen: true,
        }
    }

    fn next(mut self) -> Self {
        self.number *= self.digits_sum;
        self
    }
}

// We want the smallest number first, but `BinaryHeap` is a max first.
impl PartialOrd for PowerSum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.number
                .cmp(&other.number)
                .reverse()
                .then(self.digits_sum.cmp(&other.digits_sum)),
        )
    }
}

impl Ord for PowerSum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number
            .cmp(&other.number)
            .reverse()
            .then(self.digits_sum.cmp(&other.digits_sum))
    }
}

/// The number 512 is interesting because it is equal to the sum of its digits raised to some
/// power: 5 + 1 + 2 = 8, and 83 = 512. Another example of a number with this property is 614656
/// = 284.
///
/// We shall define an to be the nth term of this sequence and insist that a number must contain
/// at least two digits to have a sum.
///
/// You are given that a2 = 512 and a10 = 614656.
///
/// Find a30.
fn main() {
    // To fix the maximum digits sum, find the max number of digits in a u64, then multiply by 9.
    // 20 digits * 9 = 180.
    let max_sum_digits = std::u64::MAX.nbr_decimal_digits() as u64 * 9;
    let mut queue: BinaryHeap<PowerSum> =
        BinaryHeap::from_iter((2..=max_sum_digits).map(PowerSum::new));

    let answer = std::iter::from_fn(|| {
        while let Some(n) = queue.pop() {
            queue.push(n.next());
            if n.number.into_decimal_digits().sum::<u64>() == n.digits_sum {
                return Some(n.number);
            }
        }
        None
    })
    .nth(INPUT - 1)
    .unwrap();

    println!("The answer is: {}", answer);
}
