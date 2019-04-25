use std::fs;

const FILENAME: &str = "inputs/p042_words.txt";

fn string_to_num(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .filter_map(|&c| {
            if b'A' <= c && c <= b'Z' {
                Some((c + 1 - b'A') as usize)
            } else {
                None
            }
        })
        .sum()
}

fn is_triangle_number(n: usize) -> bool {
    let triangle_number = (1..)
        .map(|x| x * (x + 1) / 2)
        .skip_while(|&x| x < n)
        .next()
        .unwrap_or(1);

    triangle_number == n
}

fn main() -> std::io::Result<()> {
    let input_file = fs::read_to_string(FILENAME)?;

    let answer = input_file
        .split(',')
        .filter(|word| is_triangle_number(string_to_num(word)))
        .count();

    println!("Answer: {}", answer);

    Ok(())
}
