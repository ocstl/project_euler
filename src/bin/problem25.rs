use num::BigUint;
use project_euler::fibonacci::Fibonacci;

const INPUT: usize = 1000;

fn main() {
    let lower_limit = &(1..INPUT).fold(BigUint::from(1_usize), |acc: BigUint, _| acc * 10_usize);

    let answer = Fibonacci::<BigUint>::new()
        .enumerate()
        .find(|(_, x)| x >= lower_limit)
        .unwrap();

    println!("Term {} is {}", answer.0 + 1, answer.1);
}
