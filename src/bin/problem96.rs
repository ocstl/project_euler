use std::collections::HashSet;
use std::fs;

const BASE: u32 = 10;
const FILENAME: &str = "inputs/p096_sudoku.txt";
const SIZE: usize = 9;

#[derive(Debug, Clone)]
struct SudokuPuzzle(Box<[u32]>);

impl SudokuPuzzle {
    fn line(&self, idx: usize) -> impl Iterator<Item = &u32> + '_ {
        self.0
            .iter()
            .skip(Self::idx_to_line_nbr(idx) * SIZE)
            .take(SIZE)
    }

    fn column(&self, idx: usize) -> impl Iterator<Item = &u32> + '_ {
        self.0
            .iter()
            .skip(Self::idx_to_col_nbr(idx))
            .step_by(SIZE)
            .take(9)
    }

    fn block(&self, idx: usize) -> impl Iterator<Item = &u32> + '_ {
        let line_idx = Self::idx_to_line_nbr(idx);
        let line_idx = line_idx - (line_idx % 3);

        let col_idx = Self::idx_to_col_nbr(idx);
        let col_idx = col_idx - (col_idx % 3);

        (line_idx..)
            .take(3)
            .flat_map(move |l| self.0.iter().skip(l * SIZE).skip(col_idx).take(3))
    }

    fn idx_to_line_nbr(idx: usize) -> usize {
        idx / SIZE
    }

    fn idx_to_col_nbr(idx: usize) -> usize {
        idx % SIZE
    }
}

/// The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty
/// different Su Doku puzzles ranging in difficulty, but all with unique solutions (the first
/// puzzle in the file is the example above).
///
/// By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner
/// of each solution grid; for example, 483 is the 3-digit number found in the top left corner of
/// the solution grid above.
fn main() {
    let puzzles = parse_input(FILENAME);

    let answer: u32 = puzzles.into_iter().map(|puzzle| {
        let solved = recursive_solve(puzzle).unwrap();
        solved.line(0).take(3).fold(0, |acc, &digit| acc * BASE + digit)
    }).sum();

    println!("The answer is: {}", answer);
}

fn recursive_solve(puzzle: SudokuPuzzle) -> Option<SudokuPuzzle> {
    if let Some(idx) =
        puzzle
            .0
            .iter()
            .enumerate()
            .find_map(|(idx, &digit)| if digit == 0 { Some(idx) } else { None })
    {
        let used_digits: HashSet<&u32> = puzzle
            .line(idx)
            .chain(puzzle.column(idx))
            .chain(puzzle.block(idx))
            .collect();

        (1..=9)
            .filter(|digit| !used_digits.contains(digit))
            .find_map(|digit| {
                let mut new_puzzle = puzzle.clone();
                new_puzzle.0[idx] = digit;
                recursive_solve(new_puzzle)
            })
    } else {
        Some(puzzle)
    }
}

fn parse_input(filename: &str) -> Vec<SudokuPuzzle> {
    let s = fs::read_to_string(filename).unwrap();

    let mut lines = s.lines();
    let mut puzzles = Vec::new();

    while lines.next().unwrap_or("").contains("Grid") {
        let sudoku = lines
            .by_ref()
            .take(SIZE)
            .flat_map(|l| l.chars().map(|c| c.to_digit(BASE).unwrap()))
            .collect::<Vec<u32>>()
            .into_boxed_slice();
        puzzles.push(SudokuPuzzle(sudoku));
    }

    puzzles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_column_block() {
        let v = SudokuPuzzle(
            vec![
                1, 2, 3, 1, 2, 3, 1, 2, 3, 4, 5, 6, 4, 5, 6, 4, 5, 6, 7, 8, 9, 7, 8, 9, 7, 8, 9, 1,
                2, 3, 1, 2, 3, 1, 2, 3, 4, 5, 6, 4, 5, 6, 4, 5, 6, 7, 8, 9, 7, 8, 9, 7, 8, 9, 1, 2,
                3, 1, 2, 3, 1, 2, 3, 4, 5, 6, 4, 5, 6, 4, 5, 6, 7, 8, 9, 7, 8, 9, 7, 8, 9,
            ]
            .into_boxed_slice(),
        );

        assert_eq!(
            v.line(0).cloned().collect::<Vec<u32>>(),
            vec![1, 2, 3, 1, 2, 3, 1, 2, 3]
        );
        assert_eq!(
            v.column(0).cloned().collect::<Vec<u32>>(),
            vec![1, 4, 7, 1, 4, 7, 1, 4, 7]
        );

        assert_eq!(
            v.block(0).cloned().collect::<Vec<u32>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
