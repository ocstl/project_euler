use project_euler::unsigned::UnsignedInteger;

const BASE: usize = 10;
const MULTIPLES: [usize; 5] = [2, 3, 4, 5, 6];

fn multiple_permutation(n: usize, multiples: &[usize]) -> bool {
    let mut digits = n.to_radix_le(BASE);
    digits.sort();

    multiples.iter().all(|&multiple| {
        let mut m = (n * multiple).to_radix_le(BASE);
        m.sort();
        m == digits
    })
}

/// It can be seen that the number, 125874, and its double, 251748, contain exactly the same
/// digits, but in a different order.
///
/// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
/// digits.
fn main() {
    let answer = (1..)
        .find(|&n| multiple_permutation(n, &MULTIPLES))
        .unwrap();

    println!("Answer: {}", answer);
}
