use itertools::Itertools;
use project_euler::ciphers::frequencies::letter_frequencies;
use project_euler::ciphers::xor_cipher::xor_cipher;
use std::fs;

const INPUT: &str = "inputs/p059_cipher.txt";
const LEN_KEY: usize = 3;

fn main() {
    let ciphertext: Vec<u8> = fs::read_to_string(INPUT)
        .unwrap()
        .split(',')
        .map(|s| u8::from_str_radix(s, 10).unwrap())
        .collect();

    // Build an iterator over the possible keys (3 lowercase letters).
    let candidate_keys = (0..LEN_KEY).map(|_| b'a'..=b'z').multi_cartesian_product();

    let mut max_fit = core::f64::MIN;
    let mut key = Vec::new();

    for candidate in candidate_keys {
        let plaintext_value = xor_cipher(&ciphertext, &candidate)
            .into_iter()
            .map(letter_frequencies)
            .sum();
        if plaintext_value > max_fit {
            max_fit = plaintext_value;
            key = candidate;
        }
    }

    // Convert the message into plaintext (u8 actually), then add up the ASCII values.
    let answer: usize = xor_cipher(&ciphertext, &key)
        .into_iter()
        .map(usize::from)
        .sum();
    println!("Answer: {}", answer);
}
