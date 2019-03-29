use num::{Integer, Unsigned, One};

pub struct CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + One,
{
    current: T,
}

impl<T> CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + One,
{
    pub fn new(init: T) -> CollatzSequence<T> {
        CollatzSequence { current: init }
    }

    pub fn len(init: T) -> usize {
        Self::new(init).count() + 1
    }
}

impl<T> Iterator for CollatzSequence<T>
where
    T: Copy + From<u8> + Integer + Unsigned + One,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == T::one() {
            return None
        } else if self.current.is_even() {
            self.current = self.current / T::from(2);
        } else {
            self.current = self.current * T::from(3) + T::one();
        }

        Some(self.current)
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