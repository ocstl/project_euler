use num::{One, Unsigned, Zero};
use radixal::IntoDigits;
use std::ops::{AddAssign, RemAssign};

const DIGITS: u32 = 123_456_789;
const FILTER: u32 = 1_000_000_000;
const LOG_PHI: f64 = 0.208_987_640_249_978_73;
const LOG_SQRT5: f64 = 0.349_485_002_168_009_4;

fn log10_fibonacci(n: usize) -> f64 {
    (n as f64) * LOG_PHI - LOG_SQRT5
}

#[derive(Default)]
pub struct ModFibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T> + RemAssign<&'a T>,
{
    current: T,
    next: T,
    modulus: T,
}

impl<T> ModFibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T> + RemAssign<&'a T>,
{
    pub fn new(modulus: T) -> Self {
        ModFibonacci {
            current: T::zero(),
            next: T::one(),
            modulus,
        }
    }
}

impl<T> Iterator for ModFibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T> + RemAssign<&'a T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next += &self.current;
        self.next %= &self.modulus;
        std::mem::swap(&mut self.current, &mut self.next);

        Some(self.current.clone())
    }
}

/// The Fibonacci sequence is defined by the recurrence relation:
///
/// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
///
/// It turns out that F541, which contains 113 digits, is the first Fibonacci number for which the
/// last nine digits are 1-9 pandigital (contain all the digits 1 to 9, but not necessarily in
/// order). And F2749, which contains 575 digits, is the first Fibonacci number for which the
/// first nine digits are 1-9 pandigital.
///
/// Given that Fk is the first Fibonacci number for which the first nine digits AND the last nine
/// digits are 1-9 pandigital, find k.
fn main() {
    // Using BigUint is much too slow. So, break it apart into two easily handled pieces.
    // Use a `f64` to have enough precision. By using log10, we prevent overflow, and we can
    // "easily" grab only the first nine digits.
    let first_digits_pan_digital = |n: usize| -> bool {
        let f = log10_fibonacci(n);
        let first_digits = 10.0_f64.powf(f.fract() + 8.0).floor() as u32;
        first_digits.is_decimal_permutation(DIGITS)
    };

    let fibonacci = ModFibonacci::new(FILTER);
    // Remember to account for 0-indexing.
    let answer = fibonacci
        .enumerate()
        .find_map(|(idx, n)| {
            if n.is_decimal_permutation(DIGITS) && first_digits_pan_digital(idx + 1) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .unwrap();

    println!("The answer is: {}", answer);
}
