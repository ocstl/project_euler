fn factorial(x: usize) -> usize {
    (1..=x).product()
}

fn sum_factorial_digits(mut x: usize) -> usize {
    let mut sum = 0;
    while x != 0 {
        sum += factorial(x % 10);
        x /= 10;
    }

    sum
}

fn main() {
    /* Find an upper limit to the numbers. */
    let mut upper_limit = 0;
    while upper_limit <= sum_factorial_digits(upper_limit) {
        upper_limit = upper_limit * 10 + 9;
    }

    /* Find the sum of numbers that are equal to the sum of the factorial of their digits, except
     * 1 and 2. */
    let answer: usize =
        (3..upper_limit).filter(|&x| x == sum_factorial_digits(x))
            .sum();

    println!("Answer: {}", answer);
}