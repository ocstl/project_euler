use num::BigUint;

const INPUT: usize = 100;

trait Power {
    fn pow(&self, power: usize) -> Self;
}

impl Power for BigUint {
    fn pow(&self, power: usize) -> Self {
        core::iter::repeat(self).take(power).product()
    }
}

fn main() {
    let answer: u64 = (1..INPUT)
        .flat_map(|a| {
            (1..INPUT).map(move |b| {
                BigUint::from(a)
                    .pow(b)
                    .to_radix_le(10)
                    .into_iter()
                    .map(u64::from)
                    .sum()
            })
        })
        .max()
        .unwrap();

    println!("Answer: {}", answer);
}
