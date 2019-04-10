use num::{Unsigned, Integer, Bounded, CheckedAdd, Zero, One};
use crate::primes::Primes;

pub fn factorize<T>(x: T) -> Vec<T>
where
    T: Copy + From<u8> + Unsigned + Bounded + Integer + CheckedAdd + PartialOrd + Zero + One,
{

    let mut primes = Primes::new();
    let mut factors = Vec::new();
    let mut current: T;

    let mut number = x;
    while number > T::one() {
        current = primes.next().unwrap();
        while number % current == T::zero() {
            factors.push(current);
            number = number / current;
        }
    }

    factors
}