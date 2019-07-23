pub trait UnsignedInteger: Sized {
    fn to_radix_le(&self, base: Self) -> Vec<Self>;

    fn to_radix_be(&self, base: Self) -> Vec<Self> {
        let mut v = self.to_radix_le(base);
        v.reverse();
        v
    }

    fn nbr_digits(&self, base: Self) -> usize {
        self.to_radix_le(base).len()
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
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_unsigned_integer! { $t }
        impl_unsigned_integer! { $($ts),+ }
    };
}

impl_unsigned_integer!(u8, u16, u32, u64, u128, usize);
