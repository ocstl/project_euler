extern crate num;

use num::{Integer, CheckedAdd, Bounded};

#[derive(Clone, Default)]
pub struct Primes<T>
where
    T: Copy + From<u8> + PartialOrd + Integer + CheckedAdd + Bounded,
{
    primes: Vec<T>,
}

impl<T> Primes<T>
where
    T: Copy + From<u8> + PartialOrd + Integer + CheckedAdd + Bounded,
{
    pub fn new() -> Primes<T> {
        Primes { primes: Vec::new() }
    }

    pub fn check_primeness(&mut self, x: T) -> bool {
        if self.primes.is_empty() {
            self.next();
        }

        while x > *self.primes.last().unwrap() {
            self.next();
        }

        self.primes.contains(&x)
    }

    #[inline]
    fn two() -> T { T::from(2) }

    #[inline]
    fn three() -> T { T::from(3) }
}

impl<T> Iterator for Primes<T>
where
    T: Copy + From<u8> + PartialOrd + Integer + CheckedAdd + Bounded,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_prime = match self.primes.len() {
            0 => Some(Primes::two()),
            1 => Some(Primes::three()),
            _ => num::range_step(
                    *self.primes.last().unwrap() + Self::two(),
                        T::max_value(),
                        Self::two())
                    .find(|&x| !self.primes.iter().any(|&y| (x % y).is_zero())),
        };

        if let Some(x) = new_prime {
            self.primes.push(x);
        }

        new_prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn first_prime_u8() {
        let actual: Option<u8> = Primes::new().next();
        let expected: Option<u8> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn first_prime_u16() {
        let actual: Option<u16> = Primes::new().next();
        let expected: Option<u16> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn first_prime_u32() {
        let actual: Option<u32> = Primes::new().next();
        let expected: Option<u32> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn first_prime_u64() {
        let actual: Option<u64> = Primes::new().next();
        let expected: Option<u64> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn first_prime_u128() {
        let actual: Option<u128> = Primes::new().next();
        let expected: Option<u128> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn first_prime_usize() {
        let actual: Option<usize> = Primes::new().next();
        let expected: Option<usize> = Some(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn second_prime_usize() {
        let actual: Option<usize> = Primes::new().nth(1);
        let expected: Option<usize> = Some(3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn third_prime_usize() {
        let actual: Option<usize> = Primes::new().nth(2);
        let expected: Option<usize> = Some(5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn prime_larger_than_u8() {
        /* The 54th prime being 257, our iterator should return a None. */
        let actual: Option<u8> = Primes::new().nth(54);
        let expected: Option<u8> = None;

        assert_eq!(actual, expected);
    }
}
