use radixal::IntoDigits;

const MIN: u32 = 100;
const MAX: u32 = 999;

/// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    let answer: u32 = (MIN..=MAX)
        .map(|x| {
            (MIN..=x)
                .rev()
                .map(|y| x * y)
                .find(|&x| x.is_decimal_palindrome())
                .unwrap_or(0)
        })
        .max()
        .expect("Did not find any palindrome product.");

    println!("Answer: {}", answer);
}
