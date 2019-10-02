use itertools::Itertools;
use std::iter::once;

const CUBE_FACES: usize = 6;
const CUBE_VALUES: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const SQUARES: [u32; 9] = [1, 4, 9, 16, 25, 36, 49, 64, 81];

// Add `6` or `9` since we can reverse the orientation.
fn cube_faces() -> impl Iterator<Item = Vec<u32>> {
    CUBE_VALUES
        .iter()
        .cloned()
        .combinations(CUBE_FACES)
        .map(|mut v| {
            if v.contains(&6) {
                v.push(9);
            } else if v.contains(&9) {
                v.push(6);
            }

            v
        })
}

fn contains_all_squares(cubes: &(Vec<u32>, Vec<u32>)) -> bool {
    let (cube1, cube2) = cubes;

    let all_numbers: Vec<u32> = cube1
        .iter()
        .flat_map(|v1| {
            cube2
                .iter()
                .flat_map(move |v2| once(v1 * 10 + v2).chain(once(v2 * 10 + v1)))
        })
        .collect();

    SQUARES.iter().all(|s| all_numbers.contains(s))
}

/// Each of the six faces on a cube has a different digit (0 to 9) written on it; the same is
/// done to a second cube. By placing the two cubes side-by-side in different positions we can
/// form a variety of 2-digit numbers.
///
/// In fact, by carefully choosing the digits on both cubes it is possible to display all of the
/// square numbers below one-hundred: 01, 04, 09, 16, 25, 36, 49, 64, and 81.
///
/// In determining a distinct arrangement we are interested in the digits on each cube, not the
/// order:
/// * {1, 2, 3, 4, 5, 6} is equivalent to {3, 6, 4, 1, 2, 5}
/// * {1, 2, 3, 4, 5, 6} is distinct from {1, 2, 3, 4, 5, 9}
///
/// But because we are allowing 6 and 9 to be reversed, the two distinct sets in the last example
/// both represent the extended set {1, 2, 3, 4, 5, 6, 9} for the purpose of forming 2-digit
/// numbers.
///
/// How many distinct arrangements of the two cubes allow for all of the square numbers to be
/// displayed?
fn main() {
    let all_pairs =
        cube_faces().flat_map(|cube1| cube_faces().map(move |cube2| (cube1.clone(), cube2)));

    // Divide by 2 since half the combinations are just the two dices reversed. While we could
    // try to skip these, this is a very simple solution.
    let answer = all_pairs
        .filter(|cubes| contains_all_squares(cubes))
        .count()
        / 2;

    println!("The answer is: {}", answer);
}
