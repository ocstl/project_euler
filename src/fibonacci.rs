extern crate num;

use num::{Unsigned, Zero, One};

#[derive(Default)]
pub struct Fibonacci<T: Copy + Unsigned + Zero + One> {
    current: T,
    next: T,
}

impl<T: Copy + Unsigned + Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T: Copy + Unsigned + Zero + One> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}
