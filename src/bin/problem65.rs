use num::{BigInt, BigRational, One, Zero};

const BASE: u32 = 10;
const INPUT: usize = 100;

// Generator of the partial denominators for the continued fraction representation of e (Euler's
// constant).
fn partial_denominator_e() -> impl Iterator<Item = u128> {
    let mut idx = 0;
    let mut n = 0;

    std::iter::from_fn(move || {
        idx += 1;
        match idx {
            1 => Some(2),
            x if x % 3 == 0 => {
                n += 2;
                Some(n)
            }
            _ => Some(1),
        }
    })
}

// Using two ratios avoids developing the continued fraction.
struct Convergent<T: Iterator<Item = u128>> {
    last_ratio: BigRational,
    ratio: BigRational,
    partial_denominators: T,
}

impl<'a, T: Iterator<Item = u128>> Convergent<T> {
    fn new(sequence: T) -> Self {
        Convergent {
            last_ratio: BigRational::new_raw(BigInt::zero(), BigInt::one()),
            ratio: BigRational::new_raw(BigInt::one(), BigInt::zero()),
            partial_denominators: sequence,
        }
    }
}

impl<T: Iterator<Item = u128>> Iterator for Convergent<T> {
    type Item = BigRational;

    fn next(&mut self) -> Option<Self::Item> {
        // Replace `unwrap` with error handling.
        let a = BigInt::from(self.partial_denominators.next().unwrap());

        let num = self.ratio.numer() * &a + self.last_ratio.numer();
        let den = self.ratio.denom() * &a + self.last_ratio.denom();

        std::mem::swap(&mut self.ratio, &mut self.last_ratio);
        self.ratio = BigRational::new(num, den);
        Some(self.ratio.clone())
    }
}

/// Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
fn main() {
    let convergent = Convergent::new(partial_denominator_e()).nth(INPUT - 1).unwrap();
    let answer: usize = convergent
        .numer()
        .to_radix_le(BASE)
        .1
        .iter()
        .map(|&x| usize::from(x))
        .sum();

    println!("The answer is: {}", answer);
}
