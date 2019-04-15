use permutohedron::LexicalPermutation;
use std::collections::HashSet;

const INPUT: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn slice_to_nbr(slice: &[usize]) -> usize {
    slice.iter().fold(0, |acc, x| acc * 10 + x)
}

fn main() {
    let mut data = INPUT;
    let length = data.len();
    let mut solutions = HashSet::new();

    /* Brute force approach. We need to use a HashSet to filter the results, as permutations yield
       pairs. */
    loop {
        for idx in 1..length {
            let a = slice_to_nbr(&data[..idx]);
            for idy in idx+1..length {
                let b = slice_to_nbr(&data[idx..idy]);
                let c = slice_to_nbr(&data[idy..]);
                if a * b == c {
                    solutions.insert(c);
                }
            }
        }
        if !data.next_permutation() {
            break
        }
    }

    let answer: usize = solutions.iter().sum();
    println!("Answer: {}", answer);
}