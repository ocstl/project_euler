use std::iter::{once, successors};

/// A tuple representing a pythagorean triple.
#[derive(Clone, Copy)]
pub struct PythagoreanTriple(u64, u64, u64);

impl PythagoreanTriple {
    /// Sum of the three integers.
    pub fn sum(self) -> u64 {
        self.0 + self.1 + self.2
    }

    /// Iterator over all multiples of this triple.
    pub fn multiples(self) -> impl Iterator<Item = PythagoreanTriple> {
        successors(Some(self), move |&previous| Some(previous + self))
    }
}

impl std::ops::Add<Self> for PythagoreanTriple {
    type Output = PythagoreanTriple;

    fn add(self, rhs: Self) -> Self {
        PythagoreanTriple(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

/// A tuple representing a primitive pythagorean triple.
#[derive(Clone, Copy)]
pub struct PrimitivePythagoreanTriple(u64, u64, u64);

impl Default for PrimitivePythagoreanTriple {
    fn default() -> Self {
        PrimitivePythagoreanTriple(3, 4, 5)
    }
}

impl PrimitivePythagoreanTriple {
    /// Create a `PythagoreanTriple` from this primitive.
    pub fn to_pythagorean_triple(self) -> PythagoreanTriple {
        PythagoreanTriple(self.0, self.1, self.2)
    }

    // By Berggren (1934), we can generate all possible primitives by three linear transformations.
    pub fn next_primitives(self) -> impl Iterator<Item = PrimitivePythagoreanTriple> {
        let a = self.0;
        let b = self.1;
        let c = self.2;

        // Use `c` first to ensure there is no underflow.
        let first =
            PrimitivePythagoreanTriple(2 * c - 2 * b + a, 2 * c - b + 2 * a, 3 * c - 2 * b + 2 * a);
        let second =
            PrimitivePythagoreanTriple(2 * c + 2 * b + a, 2 * c + b + 2 * a, 3 * c + 2 * b + 2 * a);
        let third =
            PrimitivePythagoreanTriple(2 * c + 2 * b - a, 2 * c + b - 2 * a, 3 * c + 2 * b - 2 * a);

        once(first).chain(once(second)).chain(once(third))
    }
}
