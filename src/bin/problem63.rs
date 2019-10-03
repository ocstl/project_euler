use radixal::IntoDigits;

/// How many n-digit positive integers exist which are also an nth power?
fn main() {
    // Only nth power of single digit numbers can yield n-digit numbers. Use a u128 because of
    // overflow when using a u64.
    let bases = 1_u128..10;

    // Once we fall behind (n-1 digit number at nth power), we can never get back.
    let nbr_powers = |base: u128| -> u32 {
        (1..)
            .take_while(|&power| base.pow(power).nbr_decimal_digits() == power as usize)
            .last()
            .unwrap()
    };

    let answer: u32 = bases.map(nbr_powers).sum();
    println!("The answer is: {}", answer);
}
