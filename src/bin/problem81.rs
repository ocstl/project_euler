use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

const BASE: u32 = 10;
const FILENAME: &str = "inputs/p081_matrix.txt";

type Coordinates = (usize, usize);
type Matrix = HashMap<Coordinates, u64>;
type CumulativePath = (Reverse<u64>, Coordinates);

fn right_and_down(c: Coordinates) -> [Coordinates; 2] {
    [(c.0 + 1, c.1), (c.0, c.1 + 1)]
}

/// Find the minimal path sum, in matrix.txt, a text file containing a 80 by 80 matrix, from the
/// top left to the bottom right by only moving right and down.
fn main() {
    let input = fs::read_to_string(FILENAME).expect("Missing file.");

    // Convert to a HashMap for lookups.
    let matrix: Matrix = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.split(',')
                .enumerate()
                .map(move |(x, n)| ((x, y), u64::from_str_radix(n, BASE).unwrap()))
        })
        .collect();

    // Find the bottom right coordinate.
    let bottom_right = *matrix.keys().max().unwrap();

    // Implement DFS (use a Reverse<u64> to `pop` the shortest path first).
    let mut heap: BinaryHeap<CumulativePath> = BinaryHeap::new();
    let top_left = matrix.keys().min().unwrap();
    let initial = (Reverse(*matrix.get(top_left).unwrap()), *top_left);
    heap.push(initial);

    // Keep track of more previously visited paths, so we can flush inefficient ones.
    let mut visited = Matrix::new();

    while let Some(current) = heap.pop() {
        let s = (current.0).0;
        let position = current.1;

        // Ignore inefficient paths.
        if let Some(v) = visited.get(&position) {
            if s >= *v {
                continue;
            }
        } else {
            visited.insert(position, s);
        }

        if position == bottom_right {
            println!("The answer is: {}", s);
            break;
        }

        // Add new paths when possible.
        let [right, bottom] = right_and_down(position);
        if let Some(v) = matrix.get(&right) {
            heap.push((Reverse(s + v), right));
        }
        if let Some(v) = matrix.get(&bottom) {
            heap.push((Reverse(s + v), bottom));
        }
    }
}
