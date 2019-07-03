extern crate primal;

/// What is the largest prime factor of the number 600851475143 ?
const INPUT: usize = 600_851_475_143;

fn main() {
    let mut iter = primal::Primes::all();

    let mut n = INPUT;
    let mut answer = 0;
    while n > 1 {
        answer = iter.next().unwrap();
        while n % answer == 0 {
            n /= answer;
        }
    }
    println!("Answer: {}", answer);
}
