use counter::Counter;
use project_euler::factors::factorize;

const INPUT: usize = 10000;

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
    let answer: usize = (2..INPUT)
        .filter(|&x| {
            let y = sum_proper_divisors(x);
            y != x && x == sum_proper_divisors(y)
        })
        .sum();

    println!("Answer: {}", answer);
}