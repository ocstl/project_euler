use permutohedron::LexicalPermutation;

const INPUT: [u8; 10] =  [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let mut data = INPUT;

    // Try to find out why using the iterator trait doesn't work.
    for _ in 0..999_999 {
        data.next_permutation();
    }

    println!("Answer: {:?}", data);
}