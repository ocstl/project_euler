use project_euler::primes::Primes;

/* Since any 9-digital and 8-digital number is divisible by 3 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 ==
 * 36 % 3 == 0, and 36 + 9 % 3 == 0), we need only explore up to 7 digits numbers. */
const INPUT: usize = 7_654_321;

fn to_digits(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![0]
    } else {
        let mut n = n;
        core::iter::from_fn(move || {
            if n > 0 {
                let rem = n % 10;
                n /= 10;
                Some(rem)
            } else {
                None
            }
        })
        .collect()
    }
}

fn is_pandigital(n: usize) -> bool {
    let digits = to_digits(n);

    (1..=digits.len()).all(|x| digits.contains(&x))
}

fn main() {
    /* Really need a better prime generator. */
    let answer = Primes::new()
        .take_while(|&x| x <= INPUT)
        .filter(|&x| is_pandigital(x))
        .max()
        .unwrap();

    println!("Answer: {}", answer);
}
