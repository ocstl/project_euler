use num::BigUint;

const INPUT: usize = 1000;

fn main() {
    let number = format!("{}", BigUint::from(1_u8) << INPUT);
    let answer: u32 = number.chars().map(|x| x.to_digit(10).unwrap()).sum();

    println!("Answer: {}", answer);
}
