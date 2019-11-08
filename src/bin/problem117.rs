const ROW_SIZE: usize = 50;

struct CountingBlock {
    counts: Vec<usize>,
}

impl CountingBlock {
    fn new() -> Self {
        CountingBlock { counts: Vec::new() }
    }
}

impl Iterator for CountingBlock {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_count = match self.counts.len() {
            // A solution with all grey squares counts.
            0 => 1,
            1 => 1,
            // Start with a grey (1), red (2), green(3) or blue (4) tile, then see how many
            // possibilities for each.
            _ => self.counts.iter().rev().take(4).sum(),
        };

        self.counts.push(new_count);
        self.counts.last().cloned()
    }
}

/// Using a combination of grey square tiles and oblong tiles chosen from: red tiles (measuring
/// two units), green tiles (measuring three units), and blue tiles (measuring four units), it is
/// possible to tile a row measuring five units in length in exactly fifteen different ways.
///
/// How many ways can a row measuring fifty units in length be tiled?
fn main() {
    let answer = CountingBlock::new().nth(ROW_SIZE).unwrap();
    println!("The answer is: {}", answer);
}
