use project_euler::tools::ReverseInteger;

const MIN: u32 = 100;
const MAX: u32 = 999;

fn main() {
    // Find the largest palindrome that is the product of two 3-digit numbers.
    let answer: u32 = (MIN..=MAX)
        .map(|x| {
            (MIN..=MAX)
                .rev()
                .map(|y| x * y)
                .find(ReverseInteger::is_palindrome_base10)
                .unwrap_or(0)
        })
        .max()
        .expect("Did not find any palindrome product.");

    println!("Answer: {}", answer);
}
