use num::rational::Ratio;
use num::Integer;

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
        let a = self.sequence.next().unwrap();
        let numer = a.clone() * self.ratio.numer().clone() + self.last_ratio.numer().clone();
        let denom = a * self.ratio.denom().clone() + self.last_ratio.denom().clone();

        std::mem::swap(&mut self.ratio, &mut self.last_ratio);
        self.ratio = Ratio::new(numer, denom);
        Some(self.ratio.clone())
    }
}
