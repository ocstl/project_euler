use num::{Integer, One, Unsigned, Zero};

pub struct CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + Zero + One,
{
    current: T,
}

impl<T> CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + Zero + One,
{
    pub fn new(init: T) -> CollatzSequence<T> {
        CollatzSequence { current: init }
    }

    pub fn len(init: T) -> usize {
        Self::new(init).count()
    }
}

impl<T> Iterator for CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + Zero + One,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == T::zero() {
            None
        } else {
            let result = Some(self.current);
            self.current = if self.current == T::one() {
                T::zero()
            } else if self.current.is_even() {
                self.current / T::from(2)
            } else {
                self.current * T::from(3) + T::one()
            };

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_13() {
        let actual = CollatzSequence::len(13_usize);
        let expected = 10_usize;

        assert_eq!(actual, expected);
    }
}
