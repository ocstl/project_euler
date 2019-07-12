use core::ops::{AddAssign, Div, Sub};
use num::{One, Zero};

pub trait Two {
    fn two() -> Self;
}

macro_rules! two_impl {
    ($t:ty, $v:expr) => {
        impl Two for $t {
            #[inline]
            fn two() -> $t {
                $v
            }
        }
    };
}

two_impl!(u8, 2);
two_impl!(u16, 2);
two_impl!(u32, 2);
two_impl!(u64, 2);
two_impl!(u128, 2);
two_impl!(usize, 2);

#[derive(Clone, Copy)]
pub struct PolygonalNumberIterator<T>
where
    T: Zero + One + Two + Copy + AddAssign + Div<Output = T> + Sub<Output = T>,
{
    sides: T,
    current: T,
}

impl<T> PolygonalNumberIterator<T>
where
    T: Zero + One + Two + Copy + AddAssign + Div<Output = T> + Sub<Output = T>,
{
    pub fn new(sides: T) -> Self {
        PolygonalNumberIterator {
            sides,
            current: T::zero(),
        }
    }
}

impl<T> Iterator for PolygonalNumberIterator<T>
where
    T: Zero + One + Two + Copy + AddAssign + Div<Output = T> + Sub<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += T::one();
        Some(
            (self.sides - T::two()) * self.current * (self.current - T::one()) / T::two()
                + self.current,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_numbers() {
        let mut actual = PolygonalNumberIterator::new(3);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(3));
        assert_eq!(actual.next(), Some(6));
    }

    #[test]
    fn square_numbers() {
        let mut actual = PolygonalNumberIterator::new(4);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(4));
        assert_eq!(actual.next(), Some(9));
    }

    #[test]
    fn pentagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(5);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(5));
        assert_eq!(actual.next(), Some(12));
    }

    #[test]
    fn hexagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(6);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(6));
        assert_eq!(actual.next(), Some(15));
    }
    #[test]
    fn heptagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(7);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(7));
        assert_eq!(actual.next(), Some(18));
    }
    #[test]
    fn octagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(8);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(8));
        assert_eq!(actual.next(), Some(21));
    }
}
