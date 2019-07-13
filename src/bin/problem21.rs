use primal::Sieve;

const INPUT: usize = 10000;

fn sum_proper_divisors(prime_factors: &[(usize, usize)]) -> usize {
    let s = |(k, v): &(usize, usize)| -> usize { (0..=*v as u32).map(|x| k.pow(x)).sum() };
    prime_factors.iter().map(s).product()
}

/// Evaluate the sum of all the amicable numbers under 10000.
fn main() {
    let sieve = Sieve::new(INPUT);

    let wrapper = |n: usize| -> usize {
        let prime_factors = sieve.factor(n).unwrap();
        sum_proper_divisors(&prime_factors) - n
    };

    let answer: usize = (2..INPUT)
        .filter(|&x| {
            let y = wrapper(x);
            y != x && x == wrapper(y)
        })
        .sum();

    println!("Answer: {}", answer);
}
