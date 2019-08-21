use primal::Sieve;

/// Returns Euler's totient function.
///
/// The returned function:
///
/// Returns `Ok(x)` where x is the totient.
/// Returns `Err((leftover, partial factorisation))` if `n` cannot
/// be fully factored, or if `n` is zero (`leftover == 0`). A
/// number can not be completely factored if and only if the prime
/// factors of `n` are too large for this sieve, that is, if there
/// is
///
/// - a prime factor larger than `U^2`, or
/// - more than one prime factor between `U` and `U^2`
pub fn totient_function(
    limit: usize,
) -> impl Fn(usize) -> Result<usize, (usize, Vec<(usize, usize)>)> {
    let primes = Sieve::new(limit);

    move |n: usize| -> Result<usize, (usize, Vec<(usize, usize)>)> {
        Ok(primes
            .factor(n)?
            .into_iter()
            // See the proof of Euler's formula
            // (https://en.wikipedia.org/wiki/Euler%27s_totient_function#Proof_of_Euler's_product_formula)
            // Using a product of `f64` yields no appreciable improvement in performance.
            .map(|(p, x)| p.pow(x as u32) - p.pow(x as u32 - 1))
            .product())
    }
}
