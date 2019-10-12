use std::f64;
use std::fs;
use std::str::FromStr;

const FILENAME: &str = "inputs/p099_base_exp.txt";

/// Comparing two numbers written in index form like 211 and 37 is not difficult, as any
/// calculator would confirm that 211 = 2048 < 37 = 2187.
///
/// However, confirming that 632382518061 > 519432525806 would be much more difficult, as both
/// numbers contain over three million digits.
///
/// Using base_exp.txt, a 22K text file containing one thousand lines with a base/exponent pair
/// on each line, determine which line number has the greatest numerical value.
///
/// NOTE: The first two lines in the file represent the numbers in the example given above.
fn main() {
    let inputs = fs::read_to_string(FILENAME).unwrap();
    let answer = inputs
        .lines()
        .enumerate()
        // Convert to log2, and add an index starting at 1.
        .map(|(idx, line)| {
            let mut l = line.split(',');
            let (base, exp) = (
                f64::from_str(l.next().unwrap()).unwrap(),
                f64::from_str(l.next().unwrap()).unwrap(),
            );
            (idx + 1, exp * base.log2())
        })
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap()
        .0;

    println!("The answer is: {}", answer);
}
