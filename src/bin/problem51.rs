use primal::is_prime;

// Find an eight prime value family.
const BASE: u64 = 10;
const INPUT: usize = 8;

struct Family(Vec<u64>);

impl Family {
    fn new(initial: u64, step: u64) -> Self {
        // Do not use the initial value if it is too short (not the required number of digits).
        if initial > step {
            Family(vec![
                initial,
                initial + step,
                initial + 2 * step,
                initial + 3 * step,
                initial + 4 * step,
                initial + 5 * step,
                initial + 6 * step,
                initial + 7 * step,
                initial + 8 * step,
                initial + 9 * step,
            ])
        } else {
            Family(vec![
                initial + step,
                initial + 2 * step,
                initial + 3 * step,
                initial + 4 * step,
                initial + 5 * step,
                initial + 6 * step,
                initial + 7 * step,
                initial + 8 * step,
                initial + 9 * step,
            ])
        }
    }

    fn iter(&self) -> impl Iterator<Item = &u64> {
        self.0.iter()
    }

    fn first(&self) -> u64 {
        self.0[0]
    }
}

struct FamilyGenerator {
    length: u32,
    current: u64,
}

impl FamilyGenerator {
    fn new(length: u32) -> Self {
        FamilyGenerator { length, current: 0 }
    }
}

impl Iterator for FamilyGenerator {
    type Item = Family;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current == 10_u64.pow(self.length) {
            None
        } else {
            let mut n = self.current;
            let step = (0..self.length).fold(0, |acc, pos| {
                let rem = if n % BASE == 0 { 1 } else { 0 };
                n /= BASE;
                acc + rem * 10_u64.pow(pos)
            });

            // Only return `None` when we're really done.
            if step == 0 {
                self.next()
            } else {
                Some(Family::new(self.current, step))
            }
        }
    }
}

fn main() {
    let answer = (1..)
        .flat_map(FamilyGenerator::new)
        .find(|family| family.iter().filter(|&&n| is_prime(n)).count() == INPUT)
        .unwrap()
        .first();

    println!("Answer: {}", answer);
}
