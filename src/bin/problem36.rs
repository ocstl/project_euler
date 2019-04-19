const INPUT: u32 = 1_000_000;

pub enum Base {
    Binary,
    Decimal,
}

fn is_palindrome(x: u32, base: Base) -> bool {
    let s = match base {
        Base::Binary => format!("{:b}", x),
        Base::Decimal => format!("{}", x),
    };

    /* Faster than collecting into a String. */
    s.chars()
        .zip(s.chars().rev())
        .all(|(a, b)| a == b)
}

fn main () {
    let palindrome =
        |x: &u32| is_palindrome(*x, Base::Binary) && is_palindrome(*x, Base::Decimal);

    /* Since we don't tolerate leading zeroes in the binary format, we can eliminate skip even
     * numbers. */
    let answer: u32 =
        (1..INPUT)
            .step_by(2)
            .filter(palindrome)
            .sum();

    println!("Answer: {}", answer);
}