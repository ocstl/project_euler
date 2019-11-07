const ROW_SIZE: usize = 50;
const TILES: [Tile; 3] = [Tile::Red, Tile::Green, Tile::Blue];

#[derive(Clone, Copy)]
#[repr(usize)]
enum Tile {
    Red = 2,
    Green = 3,
    Blue = 4,
}

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
            self.counts.push(0);
            return self.counts.last().cloned();
        }

        let new_count: usize = self.counts[0..=(row_length - self.min_length)]
            .iter()
            .map(|c| c + 1)
            .sum::<usize>();

        self.counts.push(new_count);
        self.counts.last().cloned()
    }
}

/// A row of five grey square tiles is to have a number of its tiles replaced with coloured
/// oblong tiles chosen from red (length two), green (length three), or blue (length four).
///
/// If red tiles are chosen there are exactly seven ways this can be done.
/// If green tiles are chosen there are three ways.
/// And if blue tiles are chosen there are two ways.
///
/// Assuming that colours cannot be mixed there are 7 + 3 + 2 = 12 ways of replacing the grey
/// tiles in a row measuring five units in length.
///
/// How many different ways can the grey tiles in a row measuring fifty units in length be
/// replaced if colours cannot be mixed and at least one coloured tile must be used?
fn main() {
    let answer: usize = TILES
        .iter()
        .map(|&tile| CountingBlock::new(tile as usize).nth(ROW_SIZE).unwrap())
        .sum();

    println!("The answer is: {}", answer);
}
