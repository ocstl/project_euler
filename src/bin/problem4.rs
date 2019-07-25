use project_euler::unsigned::UnsignedInteger;

const MIN: u32 = 100;
const MAX: u32 = 999;
const BASE: u32 = 10;

/// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    let is_palindrome_base10 = |x: &u32| x.is_palindrome(BASE);

    let answer: u32 = (MIN..=MAX)
        .map(|x| {
            (MIN..=x)
                .rev()
                .map(|y| x * y)
                .find(is_palindrome_base10)
                .unwrap_or(0)
        })
        .max()
        .expect("Did not find any palindrome product.");

    println!("Answer: {}", answer);
}
