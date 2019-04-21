use num::BigUint;

const INPUT: (usize, usize) = (20, 20);

fn main() {
    let numerator = (1..=(INPUT.0 + INPUT.1)).fold(BigUint::from(1_u8), |acc, x| acc * x);
    let denominator = (1..=INPUT.0).fold(BigUint::from(1_u8), |acc, x| acc * x)
        * (1..=INPUT.1).fold(BigUint::from(1_u8), |acc, x| acc * x);

    let answer = numerator / denominator;

    println!("Answer: {}", answer);
}
