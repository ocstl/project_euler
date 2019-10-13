use peroxide::numerical::interp::lagrange_polynomial;
use peroxide::structure::vector::Vector;

/// If we are presented with the first k terms of a sequence it is impossible to say with
/// certainty the value of the next term, as there are infinitely many polynomial functions that
/// can model the sequence.
///
/// Suppose we were only given the first two terms of this sequence. Working on the principle
/// that "simple is best" we should assume a linear relationship and predict the next term to be
/// 15 (common difference 7). Even if we were presented with the first three terms, by the same
/// principle of simplicity, a quadratic relationship should be assumed.
///
/// We shall define OP(k, n) to be the nth term of the optimum polynomial generating function for
/// the first k terms of a sequence. It should be clear that OP(k, n) will accurately generate
/// the terms of the sequence for n ≤ k, and potentially the first incorrect term (FIT) will be
/// OP(k, k+1); in which case we shall call it a bad OP (BOP).
///
/// Consider the following tenth degree polynomial generating function:
///
/// u_n = 1 − n + n2 − n3 + n4 − n5 + n6 − n7 + n8 − n9 + n10
///
/// Find the sum of FITs for the BOPs.
fn main() {
    let x_vector: Vector = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0,
    ];
    let y_vector: Vector = x_vector.iter().map(|x| input_function(*x)).collect();
    let mut sum_fits = 0.0;

    for nbr_terms in 1..=x_vector.len() {
        let fitted = lagrange_polynomial(
            x_vector[0..nbr_terms].to_vec(),
            y_vector[0..nbr_terms].to_vec(),
        );

        let error = x_vector
            .iter()
            .zip(y_vector.iter())
            // Should have a perfect fit.
            .skip(nbr_terms)
            .find_map(|(&x, &y)| {
                let approximation = fitted.eval(x);
                if (approximation - y).abs() > 1e-4 {
                    Some(approximation)
                } else {
                    None
                }
            }).unwrap_or(0.0);
        sum_fits += error;
    }

    println!("The answer is: {}", sum_fits.round());
}

fn input_function(n: f64) -> f64 {
    // This could be improved, but is easily readable.
    (0..=10).map(|p| (-1.0_f64).powi(p) * n.powi(p)).sum()
}
