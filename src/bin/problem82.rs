use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

const BASE: u32 = 10;
const FILENAME: &str = "inputs/p082_matrix.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinates(usize, usize);

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Coordinates(x, y)
    }

    fn right(self) -> Option<Self> {
        Some(Coordinates::new(self.0.checked_add(1)?, self.1))
    }

    fn down(self) -> Option<Self> {
        Some(Coordinates::new(self.0, self.1.checked_add(1)?))
    }

    fn up(self) -> Option<Self> {
        Some(Coordinates::new(self.0, self.1.checked_sub(1)?))
    }

    fn neighbours(self) -> impl Iterator<Item = Self> {
        self.right()
            .into_iter()
            .chain(self.down().into_iter())
            .chain(self.up().into_iter())
    }
}

type Matrix = HashMap<Coordinates, u64>;
type CumulativePath = (Reverse<u64>, Coordinates);

/// Find the minimal path sum, in matrix.txt, a text file containing a 80 by 80 matrix, from the
/// left column to the right column by only moving right, down and up.
fn main() {
    let input = fs::read_to_string(FILENAME).expect("Missing file.");

    // Convert to a HashMap for lookups.
    let matrix: Matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split(',')
                .enumerate()
                .map(move |(x, n)| (Coordinates(x, y), u64::from_str_radix(n, BASE).unwrap()))
        })
        .collect();

    // Generate left and right columns (start and end points).
    let nbr_rows = input.lines().count();
    let nbr_columns = input.lines().next().unwrap().split(',').count();

    let left_column = (0..nbr_rows).map(|row| Coordinates::new(0, row));
    let right_column = (0..nbr_rows).map(|row| Coordinates::new(nbr_columns - 1, row));

    let get_shortest_path =
        |start: Coordinates, end: Coordinates| shortest_path(start, end, &matrix);

    let answer = left_column
        .flat_map(|start| {
            right_column
                .clone()
                .filter_map(move |end| get_shortest_path(start, end))
        })
        .min()
        .unwrap();

    println!("The answer is: {}", answer);
}

// Use DFS to find the shortest path.
fn shortest_path(start: Coordinates, end: Coordinates, matrix: &Matrix) -> Option<u64> {
    let mut paths: BinaryHeap<CumulativePath> = BinaryHeap::new();
    let initial = (Reverse(*matrix.get(&start).unwrap()), start);
    paths.push(initial);

    // Keep track of previously visited paths so we can filter out inefficient ones.
    let mut visited = Matrix::new();

    while let Some((Reverse(total), position)) = paths.pop() {
        // Filter out inefficient paths.
        if let Some(v) = visited.get(&position) {
            if total >= *v {
                continue;
            }
        }

        visited.insert(position, total);

        if position == end {
            return Some(total);
        }

        // Move to the neighbouring coordinates, if possible.
        for new_position in position.neighbours() {
            if let Some(v) = matrix.get(&new_position) {
                paths.push((Reverse(total + v), new_position));
            }
        }
    }

    None
}
