extern crate num;

use num::{Unsigned, Zero, One};

#[derive(Default)]
pub struct Fibonacci<T: Clone + Unsigned + Zero + One> {
    current: T,
    next: T,
}

impl<T: Clone + Unsigned + Zero + One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T: Clone + Unsigned + Zero + One> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current.clone() + self.next.clone();
        self.current = self.next.clone();
        self.next = new_next;

        Some(self.current.clone())
    }
}
