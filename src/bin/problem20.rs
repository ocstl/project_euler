use num::BigUint;

const INPUT: usize = 100;

fn main() {
    let answer: u32 = (1..INPUT)
        .product::<BigUint>()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    println!("Answer: {}", answer);
}
