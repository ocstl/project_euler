extern crate num;

use num::{Unsigned, Zero, One};

#[derive(Default)]
pub struct TriangleNumbers<T: Copy + Unsigned + Zero + One> {
    current: T,
    next: T,
}

impl<T: Copy + Unsigned + Zero + One> TriangleNumbers<T> {
    pub fn new() -> TriangleNumbers<T> {
        TriangleNumbers {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T: Copy + Unsigned + Zero + One> Iterator for TriangleNumbers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.current + self.next;
        self.next = self.next + T::one();

        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_numbers() {
        let mut triangles: TriangleNumbers<usize> = TriangleNumbers::new();
        assert_eq!(triangles.next(), Some(1));
        assert_eq!(triangles.next(), Some(3));
        assert_eq!(triangles.next(), Some(6));
        assert_eq!(triangles.next(), Some(10));
        assert_eq!(triangles.next(), Some(15));
        assert_eq!(triangles.next(), Some(21));
    }
}