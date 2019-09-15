const INPUT: i64 = 1_000_000;

/// Generalized because we alternate positive and negative integers (1, -1, 2, -2, ...).
struct GeneralizedPentagonalNumbers(i64);

impl GeneralizedPentagonalNumbers {
    fn new() -> Self {
        Self(0)
    }
}

impl Iterator for GeneralizedPentagonalNumbers {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= 0 {
            self.0 = -self.0 + 1;
        } else {
            self.0 = -self.0;
        }

        Some((self.0, self.0 * (3 * self.0 - 1) / 2))
    }
}

/// Rather than recomputing, use a cache. See the pentagonal number theorem for more information
/// on the recurrence relation.
struct NbrPartitions {
    partitions: Vec<i64>,
    modulus: i64,
}

impl NbrPartitions {
    // p(0) is set to 1 to simplify things.
    fn new(modulus: i64) -> Self {
        NbrPartitions {
            partitions: vec![1, 1],
            modulus,
        }
    }

    fn next(&mut self) {
        let n = self.partitions.len() as i64;
        let p = GeneralizedPentagonalNumbers::new()
            .take_while(|&(_, g)| g <= n)
            .map(|(k, g)| {
                if k % 2 == 0 {
                    -self.get((n - g) as usize)
                } else {
                    self.get((n - g) as usize)
                }
            })
            .sum::<i64>()
            % self.modulus;

        self.partitions.push(p);
    }

    fn get(&mut self, n: usize) -> i64 {
        while let None = self.partitions.get(n) {
            self.next();
        }

        *self.partitions.get(n).unwrap()
    }
}

/// Let p(n) represent the number of different ways in which n coins can be separated into piles.
/// Find the least value of n for which p(n) is divisible by one million.
fn main() {
    let mut partitions = NbrPartitions::new(INPUT);
    let answer = (2..).find(|&n| partitions.get(n) == 0).unwrap_or(0);

    println!("The answer is: {}", answer);
}

#[test]
fn five() {
    let n = NbrPartitions::new(100).get(5);
    assert_eq!(n, 7);
}

#[test]
fn generalized_pentagonal_numbers() {
    let mut g = GeneralizedPentagonalNumbers::new();
    assert_eq!(g.next(), Some((1, 1)));
    assert_eq!(g.next(), Some((-1, 2)));
    assert_eq!(g.next(), Some((2, 5)));
    assert_eq!(g.next(), Some((-2, 7)));
}
