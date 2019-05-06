use project_euler::tools::to_digits;

const MULTIPLES: [usize; 5] = [2, 3, 4, 5, 6];

fn multiple_permutation(n: usize, multiples: &[usize]) -> bool {
    let mut digits = to_digits(n);
    digits.sort();

    multiples.iter().all(|&multiple| {
        let mut m = to_digits(n * multiple);
        m.sort();
        m == digits
    })
}

fn main() {
    let answer = (1..)
        .find(|&n| multiple_permutation(n, &MULTIPLES))
        .unwrap();

    println!("Answer: {}", answer);
}
