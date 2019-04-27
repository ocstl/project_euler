fn is_pentagonal(p_n: u32) -> bool {
    /* Quadratic formula. The result should be an integer. */
    let n = (1.0 + (1.0 + 24.0 * f64::from(p_n)).sqrt()) / 6.0;
    (n - n.trunc()).abs() <  10.0 * core::f64::EPSILON
}

fn to_pentagonal(n: u32) -> u32 {
    n * (3 * n - 1) / 2
}

fn main() {
    let pair = (1..)
        .flat_map(|j| (1..j).map(move |k| (to_pentagonal(j), to_pentagonal(k))))
        .find(|&(p_j, p_k)| is_pentagonal(p_j - p_k) && is_pentagonal(p_j + p_k))
        .unwrap();

    let answer = pair.0 - pair.1;

    println!("Answer: {}", answer);
}
