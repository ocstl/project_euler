const DIGITS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    /* Since the concatenated number is the result of n > 1, it cannot be longer than 4 digits. */
    let answer = (1..=9999)
        .filter_map(|i| {
            /* Generate strings of increasing length, until we reach the maximum length of 9
             * (for 9 digits). */
            let candidate = (1..)
                .scan(String::new(), |acc, m| {
                    acc.push_str(&(i * m).to_string());
                    Some(acc.clone())
                })
                .take_while(|s| s.len() <= 9)
                .last()
                .unwrap();

            /* Only keep pandigital results. */
            if DIGITS.iter().all(|&x| candidate.contains(x)) {
                Some(candidate)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!("Answer: {}", answer);
}
