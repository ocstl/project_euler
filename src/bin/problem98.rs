use num::integer::Roots;
use permutohedron::LexicalPermutation;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

const BASE: u32 = 10;
const DIGITS: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const FILENAME: &str = "inputs/p098_words.txt";

type Mapping = HashMap<char, u32>;

struct MappingGenerator {
    letters: Vec<char>,
    values: [u32; 10],
    length: usize,
}

impl MappingGenerator {
    fn new(input: &str) -> MappingGenerator {
        let letters: Vec<char> = input
            .chars()
            .collect::<HashSet<char>>()
            .into_iter()
            .collect();

        MappingGenerator {
            length: letters.len(),
            letters,
            values: DIGITS,
        }
    }
}

impl Iterator for MappingGenerator {
    type Item = Mapping;

    fn next(&mut self) -> Option<Self::Item> {
        let last_values = self.values.to_vec();

        // Skip when the relevant part of the permutation has been seen before.
        while self.values.next_permutation() {
            if &last_values[..self.length] != &self.values[..self.length] {
                return Some(
                    self.letters
                        .iter()
                        .cloned()
                        .zip(self.values.iter().cloned())
                        .collect::<Mapping>(),
                );
            }
        }

        None
    }
}

fn word_to_number(word: &str, mapping: &Mapping) -> u32 {
    word.chars().fold(0, |mut acc, c| {
        acc = acc * BASE + *mapping.get(&c).unwrap_or(&0);
        acc
    })
}

fn is_perfect_square(n: u32) -> bool {
    n.sqrt().pow(2) == n
}

fn palindrome(string1: &str, string2: &str) -> bool {
    string1
        .chars()
        .zip(string2.chars().rev())
        .all(|(a, b)| a == b)
}

// Build word anagram pairs, unless they are palindromes.
fn word_pairs(mut anagrams: Vec<&str>) -> Vec<(&str, &str)> {
    let mut pairs = Vec::new();
    while let Some(word) = anagrams.pop() {
        for word2 in &anagrams {
            if !palindrome(word, word2) {
                pairs.push((word, *word2));
            }
        }
    }

    pairs
}

/// By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a
/// square number: 1296 = 362. What is remarkable is that, by using the same digital
/// substitutions, the anagram, RACE, also forms a square number: 9216 = 962. We shall call CARE
/// (and RACE) a square anagram word pair and specify further that leading zeroes are not
/// permitted, neither may a different letter have the same digital value as another letter.
///
/// Using words.txt, a 16K text file containing nearly two-thousand common English words, find
/// all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of
/// itself).
///
/// What is the largest square number formed by any member of such a pair?
///
/// NOTE: All anagrams formed must be contained in the given text file.
fn main() {
    let words = fs::read_to_string(FILENAME).unwrap();

    // Find all anagrams (including palindromes). We are assuming no words is present twice.
    let mut words = words.split(',').map(|word| word.trim_matches('"')).fold(
        HashMap::<Vec<char>, Vec<&str>>::new(),
        |mut acc, word| {
            let mut letters = word.chars().collect::<Vec<char>>();
            letters.sort_unstable();
            acc.entry(letters).or_default().push(word);
            acc
        },
    );

    // Filter out words without anagrams.
    words.retain(|_, words| words.len() > 1 && !palindrome(words[0], words[1]));

    // Build the word pairs. Use a BTreeMap so we can start with the longest words.
    let pairs = words.into_iter().fold(
        BTreeMap::<usize, Vec<(&str, &str)>>::new(),
        |mut acc, (letters, anagrams)| {
            let mut pairs = word_pairs(anagrams);
            acc.entry(letters.len()).or_default().append(&mut pairs);
            acc
        },
    );

    let answer: u32 = pairs
        .values()
        .rev()
        .find_map(|pairs| {
            pairs
                .iter()
                .flat_map(|(word1, word2)| {

                    // Function to eliminate mapping where the first characters of any of the two
                    // strings is 0.
                    let char1 = word1.chars().next().unwrap();
                    let char2 = word2.chars().next().unwrap();
                    let check_first_chars = move |mapping: &Mapping| {
                        mapping.get(&char1) == Some(&0) || mapping.get(&char2) == Some(&0)
                    };

                    let gen = MappingGenerator::new(word1);
                    gen.filter_map(move |mapping| {
                        if check_first_chars(&mapping) {
                            return None;
                        }

                        let n1 = word_to_number(word1, &mapping);
                        let n2 = word_to_number(word2, &mapping);

                        if is_perfect_square(n1) && is_perfect_square(n2) {
                            Some(n1.max(n2))
                        } else {
                            None
                        }
                    })
                })
                .max()
        })
        .unwrap_or(0);

    println!("The answer is: {}", answer);
}
