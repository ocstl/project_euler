extern crate num;

use crate::primes::Primes;

pub fn factorize<T>(x: T) -> Vec<T>
where
    T: Copy + From<u8> + num::Unsigned + num::Bounded + num::CheckedAdd
        + PartialOrd,
{

    let mut primes = Primes::new();
    let mut factors = Vec::new();
    let mut current: T;

    let mut number = x;
    while number > T::one() {
        current = primes.next().unwrap();
        while number % current == 0.into() {
            factors.push(current);
            number = number / current;
        }
    }

    factors
}
