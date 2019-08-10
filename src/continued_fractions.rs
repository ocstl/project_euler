use num::rational::Ratio;
use num::Integer;

/// Convergent iterator.
///
/// A continued fraction representation provides rational approximations to a number, called
/// convergents. This iterator yields the convergents for a given sequence of partial
/// denominators, as a `Ratio<T>`.
pub struct ConvergentIterator<T, U> {
    last_ratio: Ratio<T>,
    ratio: Ratio<T>,
    partial_denominators: U,
}

impl<T: Clone + Integer, U: Iterator<Item = T>> ConvergentIterator<T, U> {
    /// Build a new `ConvergentIterator` with the given sequence of partial denominators.
    pub fn new(sequence: U) -> Self {
        ConvergentIterator {
            last_ratio: Ratio::new_raw(T::zero(), T::one()),
            ratio: Ratio::new_raw(T::one(), T::zero()),
            partial_denominators: sequence,
        }
    }
}

impl<'a, T: Clone + Integer, U: Iterator<Item = T>> Iterator for ConvergentIterator<T, U> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Replace `unwrap` with error handling.
        let a = self.partial_denominators.next().unwrap();
        let numer = a.clone() * self.ratio.numer().clone() + self.last_ratio.numer().clone();
        let denom = a * self.ratio.denom().clone() + self.last_ratio.denom().clone();

        std::mem::swap(&mut self.ratio, &mut self.last_ratio);
        self.ratio = Ratio::new(numer, denom);
        Some(self.ratio.clone())
    }
}
