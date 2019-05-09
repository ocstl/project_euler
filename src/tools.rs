pub fn to_digits(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![0]
    } else {
        let mut n = n;
        core::iter::from_fn(move || {
            if n > 0 {
                let rem = n % 10;
                n /= 10;
                Some(rem)
            } else {
                None
            }
        })
        .collect()
    }
}

pub fn to_number(n: &[usize]) -> usize {
    n.iter().fold(0, |acc, x| acc * 10 + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        let actual = to_digits(123);
        let expected = vec![3, 2, 1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_to_number() {
        let tmp = [1, 2, 3];
        let actual = to_number(&tmp);
        let expected = 123;

        assert_eq!(actual, expected);
    }
}

pub trait ReverseInteger: PartialEq + Eq + Sized {
    fn reverse_base10(&self) -> Self;

    fn is_palindrome_base10(&self) -> bool {
        self.reverse_base10() == *self
    }
}

macro_rules! impl_reverse {
    ($t:ty) => {
        impl ReverseInteger for $t {
            fn reverse_base10(&self) -> $t {
                let mut n = *self;
                let mut result = 0;

                while n > 0 {
                    result = result * 10 + (n % 10);
                    n /= 10;
                }

                result
            }
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_reverse! { $t }
        impl_reverse! { $($ts),+ }
    };
}

impl_reverse!(u8, u16, u32, u64, u128, usize);
