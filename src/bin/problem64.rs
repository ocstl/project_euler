const N: u32 = 10_000;

// See https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Continued_fraction_expansion
struct Triplet {
    number: u32,
    a0: u32,
    m: u32,
    d: u32,
    a: u32,
}

impl Triplet {
    fn new(number: u32) -> Self {
        let m = 0;
        let d = 1;
        let a = f64::from(number).sqrt().floor() as u32;
        Triplet {
            number,
            a0: a,
            m,
            d,
            a,
        }
    }
}

impl Iterator for Triplet {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Back to the beginning of the period.
        if self.a == 2 * self.a0 {
            return None;
        }

        self.m = self.d * self.a - self.m;
        self.d = (self.number - self.m.pow(2)) / self.d;
        // For non-periodic continued fractions (perfect squares).
        if self.d == 0 {
            return None;
        }

        self.a = (self.a0 + self.m) / self.d;
        Some(self.a)
    }
}

struct SquareRootContinuedFraction {
    _number: u32,
    _a0: u32,
    period: Vec<u32>,
}

impl SquareRootContinuedFraction {
    fn new(number: u32) -> Self {
        let triplet = Triplet::new(number);
        let a0 = triplet.a0;
        let period = triplet.collect();

        SquareRootContinuedFraction { _number: number, _a0: a0, period }
    }

    fn period_length(&self) -> usize {
        self.period.len()
    }
}

/// All square roots are periodic when written as continued fractions. For conciseness, we use
/// the notation √23 = [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats indefinitely.
/// How many continued fractions for N <= 10000 have an odd period?
fn main() {
    let answer = (1..=N)
        .filter(|&n| SquareRootContinuedFraction::new(n).period_length() % 2 == 1)
        .count();
    println!("The answer is: {}", answer);
}
