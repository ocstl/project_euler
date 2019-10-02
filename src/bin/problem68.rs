use permutohedron::LexicalPermutation;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

const NODES: u32 = 10;

#[derive(Debug)]
struct MagicRingError;

impl fmt::Display for MagicRingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid magic ring.")
    }
}

impl Error for MagicRingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

struct MagicRing {
    inner: Vec<u32>,
    outer: Vec<u32>,
}

impl MagicRing {
    fn new(inner: &[u32], outer: &[u32]) -> Self {
        MagicRing {
            inner: inner.to_vec(),
            outer: outer.to_vec(),
        }
    }

    // Generate lines from the outside in.
    fn lines<'a>(&'a self) -> impl Iterator<Item = (u32, u32, u32)> + 'a {
        let inner = self.inner.iter().zip(self.inner.iter().cycle().skip(1));
        self.outer.iter().zip(inner).map(|(&a, (&b, &c))| (a, b, c))
    }
}

// Print the `MagicRing` by concatenating its lines (from the outside in), starting with the
// smallest value in the outside "ring".
impl fmt::Display for MagicRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<(u32, u32, u32)> = self.lines().collect();
        let min_line = lines.iter().min().unwrap();
        let idx = lines.iter().position(|line| line == min_line).unwrap();
        lines.rotate_left(idx);

        let s: String = lines
            .iter()
            .map(|(a, b, c)| format!("{}{}{}", a, b, c))
            .collect();
        write!(f, "{}", s)
    }
}

impl TryFrom<&[u32]> for MagicRing {
    type Error = MagicRingError;

    fn try_from(values: &[u32]) -> Result<Self, Self::Error> {
        let l = values.len();
        if l % 2 != 0 {
            return Err(MagicRingError);
        }

        let (outer, inner) = values.split_at(l / 2);
        let ring = MagicRing::new(inner, outer);

        let sum_line = |line: (u32, u32, u32)| line.0 + line.1 + line.2;

        {
            let mut lines = ring.lines();
            let value = sum_line(lines.next().unwrap());
            if lines.any(|line| sum_line(line) != value) {
                return Err(MagicRingError);
            }
        }

        Ok(ring)
    }
}

fn main() {
    let mut numbers: Vec<u32> = (1..=NODES).collect();
    let mut rings = Vec::new();

    while numbers.next_permutation() {
        if let Ok(ring) = MagicRing::try_from(numbers.as_slice()) {
            rings.push(format!("{}", ring));
        }
    }

    let answer = rings
        .into_iter()
        .max()
        .unwrap_or_else(|| String::from("No ring."));
    println!("The answer is {}", answer);
}
