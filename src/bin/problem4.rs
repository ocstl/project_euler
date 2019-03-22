fn main() {
    let answer = (100..999).flat_map(|x| (100..999).map(move |y| x * y))
        .filter(palindrome)
        .max()
        .unwrap_or(0);

    println!("Answer: {}", answer);
}

fn palindrome(a: &u32) -> bool {
    let temp = a.to_string();

    temp.chars().zip(temp.chars().rev())
        .all(|(x, y)| x == y)
}
