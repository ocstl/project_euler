fn is_pentagonal(p_n: u32) -> bool {
    // Quadratic formula. The result should be an integer.
    let n = (1.0 + (1.0 + 24.0 * f64::from(p_n)).sqrt()) / 6.0;
    (n - n.trunc()).abs() < 10.0 * core::f64::EPSILON
}

fn to_hexagonal(n: u32) -> u32 {
    n * (2 * n - 1)
}

fn main() {
    // Find the first triagonal number that is both pentagonal and hexagonal. Since all hexagonal
    // numbers are triagonal, we can just iterate over the hexagonal numbers. We want the "next"
    // (after 40755) one.
    let answer = (1..)
        .map(to_hexagonal)
        .skip_while(|&n| n <= 40755)
        .find(|&n| is_pentagonal(n))
        .unwrap();

    println!("Answer: {}", answer);
}
