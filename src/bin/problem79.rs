use radixal::IntoDigits;
use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "inputs/p079_keylog.txt";

#[derive(Clone, PartialEq, Eq, Hash)]
struct KeyLog([u32; 3]);

impl KeyLog {
    fn check(&self, mut password: impl Iterator<Item = u32>) -> bool {
        self.0.iter().all(|&key| password.any(|x| x == key))
    }
}

impl From<&str> for KeyLog {
    fn from(keylog: &str) -> Self {
        let mut it = keylog.chars().map(|c| c.to_digit(10).unwrap());
        KeyLog([it.next().unwrap(), it.next().unwrap(), it.next().unwrap()])
    }
}

/// A common security method used for online banking is to ask the user for three random
/// characters from a passcode. For example, if the passcode was 531278, they may ask for the
/// 2nd, 3rd, and 5th characters; the expected reply would be: 317.
///
/// The text file, keylog.txt, contains fifty successful login attempts.
///
/// Given that the three characters are always asked for in order, analyse the file so as to
/// determine the shortest possible secret passcode of unknown length.
fn main() {
    // Use a HashSet to avoid checking multiple times against the same key log.
    let keylogs = fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(KeyLog::from)
        .collect::<HashSet<KeyLog>>();

    // The problem is small enough to lends itself to brute force.
    let answer = (100..)
        .find(|password| {
            let password = password.into_decimal_digits();
            keylogs
                .iter()
                .all(|keylog| keylog.check(password.clone()))
        })
        .unwrap();

    println!("The answer is: {}", answer)
}

#[test]
fn test_key_log() {
    let keylog = KeyLog([1, 2, 3]);
    assert!(keylog.check(&[1, 2, 3]));
    assert!(!keylog.check(&[2, 1, 3]));
    assert!(keylog.check(&[1, 2, 4, 3]));
    assert!(!keylog.check(&[2, 1, 4, 3]));
}
