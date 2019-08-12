use num::integer::{Integer, Roots};
use std::iter::{once, Chain, Cycle, Once};

// See https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Continued_fraction_expansion
fn expand<T: Clone + Copy + Integer + Roots>(number: T) -> (T, Vec<T>) {
    let mut m = T::zero();
    let mut d = T::one();
    let mut a = number.sqrt();

    let a0 = a;
    let filter = a0 + a0;
    let mut period = Vec::new();

    while a != filter {
        m = d * a - m;
        d = (number - m * m) / d;

        // In the case of perfect squares, d == 0, so we can return early.
        if d.is_zero() {
            break;
        }

        a = (a0 + m) / d;
        period.push(a)
    }

    (a0, period)
}

/// Square root of integers have periodic continued fractions. In the case of perfect squares,
/// the period is of length 0.
pub struct SquareRoot<T> {
    a0: T,
    period: Vec<T>,
}

impl<T: Clone + Copy + Integer + Roots> SquareRoot<T> {
    pub fn new(number: T) -> Self {
        let (a0, period) = expand(number);

        SquareRoot { a0, period }
    }

    pub fn period_length(&self) -> usize {
        self.period.len()
    }
}

impl<T: Clone + Integer> IntoIterator for SquareRoot<T> {
    type Item = T;
    type IntoIter = Chain<Once<T>, Cycle<std::vec::IntoIter<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.a0).chain(self.period.into_iter().cycle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::continued_fractions::ContinuedFraction;
    use num::rational::Ratio;

    #[test]
    fn period_lengths() {
        assert_eq!(SquareRoot::new(2).period_length(), 1);
        assert_eq!(SquareRoot::new(3).period_length(), 2);
        assert_eq!(SquareRoot::new(4).period_length(), 0);
        assert_eq!(SquareRoot::new(5).period_length(), 1);
        assert_eq!(SquareRoot::new(6).period_length(), 2);
        assert_eq!(SquareRoot::new(7).period_length(), 4);
        assert_eq!(SquareRoot::new(8).period_length(), 2);
        assert_eq!(SquareRoot::new(9).period_length(), 0);
    }

    #[test]
    fn period_2() {
        let actual = SquareRoot::new(2);
        assert_eq!(actual.a0, 1);
        assert_eq!(actual.period, vec![2]);
    }

    #[test]
    fn convergents() {
        let s = SquareRoot::new(2_u32);
        let mut convergents = ContinuedFraction::new(s.into_iter()).into_iter();

        assert_eq!(convergents.next().unwrap(), Ratio::new(1, 1));
        assert_eq!(convergents.next().unwrap(), Ratio::new(3, 2));
        assert_eq!(convergents.next().unwrap(), Ratio::new(7, 5));
    }
}
