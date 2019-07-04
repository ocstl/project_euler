use project_euler::polygonal_numbers::PolygonalNumberIterator;

const MIN_NUMBER: u32 = 999;
const MAX_NUMBER: u32 = 10000;
const SIDES: [u32; 6] = [3, 4, 5, 6, 7, 8];

enum Numbers {
    Triangular(Vec<u32>),
    Square(Vec<u32>),
    Pentagonal(Vec<u32>),
    Hexagonal(Vec<u32>),
    Heptagonal(Vec<u32>),
    Octagonal(Vec<u32>),
}

fn main() {}
