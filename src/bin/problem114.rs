const MIN_BLOCK_LENGTH: usize = 3;
const ROW_LENGTH: usize = 50;

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

/// A row measuring seven units in length has red blocks with a minimum length of three units
/// placed on it, such that any two red blocks (which are allowed to be different lengths) are
/// separated by at least one grey square. There are exactly seventeen ways of doing this.
///
/// How many ways can a row measuring fifty units in length be filled?
fn main() {
    let answer = CountingBlock::new(MIN_BLOCK_LENGTH)
        .nth(ROW_LENGTH)
        .unwrap();
    println!("The answer is: {}", answer);
}
