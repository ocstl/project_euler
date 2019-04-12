use project_euler::primes::Primes;

/* Longest consecutive prime values from the quadratic formula: n^2 + a*n + b, with n starting at 0,
   for |a| < 1000 and |b| <= 1000.
 */

fn main() {
    let mut primes = Primes::new();
    /* Since we start at 0, b has to be prime. */
    let possible_b: Vec<isize> = primes.clone().take_while(|&x| x <= 1000).collect();

    let answer = possible_b.iter()
        /* Assuming we want more than n = 0, 1 + a + b >= 2; or a >= 1 - b. */
        .map(|b| ((1-b)..1000)
            .map(|a| {
                let quadratic_formula = |x: isize| -> isize { x.pow(2) + a * x + b };
                let length = (0..)
                    .take_while(|&x| primes.check_primeness(quadratic_formula(x)))
                    .count();
                (length, a * b)
                })
            .max().unwrap_or((0, 0)))
        .max().unwrap_or((0, 0));

    println!("Answer: {:?}", answer);
}