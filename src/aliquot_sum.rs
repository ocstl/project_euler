#[macro_export]
macro_rules! aliquot_sum_fn {
    ($limit:expr) => {
        lazy_static::lazy_static! {
            static ref SIEVE: primal::Sieve = primal::Sieve::new(INPUT);
        }

        fn aliquot_sum(number: usize) -> usize {
            if let Ok(prime_factors) = SIEVE.factor(number) {
                prime_factors
                    .into_iter()
                    .map(|(factor, exponent)| (factor.pow(1 + exponent as u32) - 1) / (factor - 1))
                    .product::<usize>()
                    - number
            } else {
                0
            }
        }
    };
}
