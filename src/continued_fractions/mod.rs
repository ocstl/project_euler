pub mod square_root;

use num::rational::Ratio;
use num::Integer;
use square_root::SquareRoot;
use std::convert::From;
use std::iter::{Chain, Cycle, Once};

/// Continued fraction.
///
/// A continued fraction representation is a sequence of partial denominators of a fraction that
/// provides rational approximations to a number, called convergents.
pub struct ContinuedFraction<T, U: Iterator<Item = T>> {
    sequence: U,
}

impl<T: Clone + Integer, U: Iterator<Item = T>> ContinuedFraction<T, U> {
    pub fn new(sequence: U) -> Self {
        ContinuedFraction { sequence }
    }
}

impl<T: Clone + Integer> From<SquareRoot<T>>
    for ContinuedFraction<T, Chain<Once<T>, Cycle<std::vec::IntoIter<T>>>>
{
    fn from(square_root: SquareRoot<T>) -> Self {
        ContinuedFraction {
            sequence: square_root.into_iter(),
        }
    }
}

impl<T: Clone + Integer, U: Iterator<Item = T>> IntoIterator for ContinuedFraction<T, U> {
    type Item = Ratio<T>;
    type IntoIter = Convergents<T, U>;

    fn into_iter(self) -> Self::IntoIter {
        Convergents::new(self.sequence)
    }
}

/// Convergent iterator.
///
/// This iterator yields the convergents for a given sequence of partial denominators, as a
/// `Ratio<T>`.
pub struct Convergents<T, U> {
    last_ratio: Ratio<T>,
    ratio: Ratio<T>,
    sequence: U,
}

impl<T: Clone + Integer, U: Iterator<Item = T>> Convergents<T, U> {
    /// Build a new `Convergents` iterator with the given sequence of partial denominators.
    fn new(sequence: U) -> Self {
        Convergents {
            last_ratio: Ratio::new_raw(T::zero(), T::one()),
            ratio: Ratio::new_raw(T::one(), T::zero()),
            sequence,
        }
    }
}

impl<T: Clone + Integer, U: Iterator<Item = T>> Iterator for Convergents<T, U> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Replace `unwrap` with error handling.
        let a = match self.sequence.next() {
            Some(x) => x,
            None => return None,
        };

        let numer = a.clone() * self.ratio.numer().clone() + self.last_ratio.numer().clone();
        let denom = a * self.ratio.denom().clone() + self.last_ratio.denom().clone();

        std::mem::swap(&mut self.ratio, &mut self.last_ratio);
        self.ratio = Ratio::new(numer, denom);
        Some(self.ratio.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_root_23() {
        let sequence = std::iter::once(4).chain([1, 3, 1, 8].iter().cloned().cycle());
        let fraction = ContinuedFraction::new(sequence);
        let mut convergents = fraction.into_iter();

        assert_eq!(convergents.next().unwrap(), Ratio::new(4, 1));
        assert_eq!(convergents.next().unwrap(), Ratio::new(5, 1));
        assert_eq!(convergents.next().unwrap(), Ratio::new(19, 4));
        assert_eq!(convergents.next().unwrap(), Ratio::new(24, 5));
        assert_eq!(convergents.next().unwrap(), Ratio::new(211, 44));
    }

    #[test]
    fn from_square_root_2() {
        let s = SquareRoot::new(2_u32);
        let mut convergents = ContinuedFraction::from(s).into_iter();

        assert_eq!(convergents.next().unwrap(), Ratio::new(1, 1));
        assert_eq!(convergents.next().unwrap(), Ratio::new(3, 2));
        assert_eq!(convergents.next().unwrap(), Ratio::new(7, 5));
    }
}
