const INPUT: usize = 1_000_000;
const MIN_BLOCK_LENGTH: usize = 50;

struct CountingBlock {
    min_length: usize,
    counts: Vec<usize>,
}

impl CountingBlock {
    fn new(min_length: usize) -> Self {
        CountingBlock {
            min_length,
            counts: Vec::new(),
        }
    }
}

impl Iterator for CountingBlock {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let row_length = self.counts.len();

        if row_length < self.min_length {
            self.counts.push(1);
            return self.counts.last().cloned();
        }

        let new_count: usize = self.counts.last().unwrap_or(&0)
            + self.counts[0..(row_length.saturating_sub(self.min_length))]
                .iter()
                .sum::<usize>()
            + 1;

        self.counts.push(new_count);
        self.counts.last().cloned()
    }
}

/// A row measuring n units in length has red blocks with a minimum length of m units placed on
/// it, such that any two red blocks (which are allowed to be different lengths) are separated by
/// at least one black square.
///
/// Let the fill-count function, F(m, n), represent the number of ways that a row can be filled.
///
/// For example, F(3, 29) = 673135 and F(3, 30) = 1089155.
///
/// That is, for m = 3, it can be seen that n = 30 is the smallest value for which the fill-count
/// function first exceeds one million.
///
/// In the same way, for m = 10, it can be verified that F(10, 56) = 880711 and F(10, 57) =
/// 1148904, so n = 57 is the least value for which the fill-count function first exceeds one
/// million.
///
/// For m = 50, find the least value of n for which the fill-count function first exceeds one
/// million.
fn main() {
    // Borrowing from problem 114 makes this easy.
    let answer = CountingBlock::new(MIN_BLOCK_LENGTH)
        .enumerate()
        .find(|&(_, count)| count > INPUT)
        .unwrap()
        .0;

    println!("The answer is: {}", answer);
}
