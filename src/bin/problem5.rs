extern crate counter;

use counter::Counter;
use project_euler::factors;

const INPUT: u64 = 20;

fn main() {
    let factors = (2..INPUT)
        .fold(Counter::new(), |acc: Counter<_>, x|
            acc | factors::factorize(x).into_iter().collect());

    let answer: u64 = factors.iter()
        .map(|(key, value)| key.pow(*value as u32))
        .product();

    println!("Answer: {}", answer);
}
