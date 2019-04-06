use counter::Counter;
use project_euler::factors::factorize;
use std::collections::HashSet;

fn sum_proper_divisors(number: usize) -> usize {
    let s = |(k, v): (&usize, &usize)| -> usize {
        (0..=*v as u32).map(|x| k.pow(x)).sum()
    };

    let prime_factors: Counter<usize, usize> = Counter::init(factorize(number));

    let sum_divisors: usize = prime_factors.iter()
        .map(s)
        .product();

    sum_divisors - number
}

fn main() {
    let abundant_numbers: Vec<usize> = (12..28123)
        .filter(|&x| sum_proper_divisors(x) > x)
        .collect();

    let sum_two_abundant_numbers: HashSet<usize> = abundant_numbers.iter()
        .flat_map(|x| abundant_numbers.iter().map(move |y| *x + y))
        .collect();

    let answer: usize = (1..=28123).collect::<HashSet<usize>>()
        .difference(&sum_two_abundant_numbers)
        .sum();

    println!("Answer: {}", answer);
}