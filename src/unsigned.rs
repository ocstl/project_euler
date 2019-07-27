pub trait UnsignedInteger: Clone + Copy + PartialEq + Eq + Sized {
    fn to_radix_le(&self, base: Self) -> Vec<Self>;

    fn to_radix_be(&self, base: Self) -> Vec<Self> {
        let mut v = self.to_radix_le(base);
        v.reverse();
        v
    }

    fn is_palindrome(&self, base: Self) -> bool {
        let v = self.to_radix_le(base);
        v.iter().rev().zip(v.iter()).all(|(a, b)| a == b)
    }

    fn nbr_digits(&self, base: Self) -> usize {
        self.to_radix_le(base).len()
    }

    fn from_radix_be(v: impl Iterator<Item = Self>, base: Self) -> Self;

    fn from_radix_le(v: impl DoubleEndedIterator<Item = Self>, base: Self) -> Self {
        Self::from_radix_be(v.rev(), base)
    }

    fn reverse_digits(&self, base: Self) -> Self {
        Self::from_radix_be(self.to_radix_le(base).into_iter(), base)
    }
}

macro_rules! impl_unsigned_integer {
    ($t:ty) => {
        impl UnsignedInteger for $t {
            fn to_radix_le(&self, base: Self) -> Vec<Self> {
                let mut n = *self;
                let mut v = Vec::new();

                while n > 0 {
                    v.push(n % base);
                    n /= base;
                }

                v
            }

            fn from_radix_be(v: impl Iterator<Item = Self>, base: Self) -> Self {
                v.fold(0, |acc, n| acc * base + n)
            }
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_unsigned_integer! { $t }
        impl_unsigned_integer! { $($ts),+ }
    };
}

impl_unsigned_integer!(u8, u16, u32, u64, u128, usize);
