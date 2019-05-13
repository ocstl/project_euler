pub fn xor_cipher(s: &[u8], key: &[u8]) -> Vec<u8> {
    s.iter()
        .zip(key.iter().cycle())
        .map(|(l, k)| l ^ k)
        .collect()
}
