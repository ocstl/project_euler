use num::{One, Unsigned, Zero};
use std::ops::AddAssign;

#[derive(Default)]
pub struct Fibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T>,
{
    current: T,
    next: T,
}

impl<T> Fibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T>,
{
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    for<'a> T: Clone + Unsigned + Zero + One + AddAssign<&'a T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next += &self.current;
        std::mem::swap(&mut self.current, &mut self.next);

        Some(self.current.clone())
    }
}
