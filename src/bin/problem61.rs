use project_euler::polygonal_numbers::PolygonalNumberIterator;
use std::mem;

const MIN_NUMBER: u32 = 999;
const MAX_NUMBER: u32 = 10000;
const SIDES: [u32; 6] = [3, 4, 5, 6, 7, 8];

#[derive(Clone, Copy, Debug)]
enum FigurateNumber {
    Triangle(u32),
    Square(u32),
    Pentagonal(u32),
    Hexagonal(u32),
    Heptagonal(u32),
    Octagonal(u32),
}

impl FigurateNumber {
    fn new(sides: u32, value: u32) -> FigurateNumber {
        match sides {
            3 => FigurateNumber::Triangle(value),
            4 => FigurateNumber::Square(value),
            5 => FigurateNumber::Pentagonal(value),
            6 => FigurateNumber::Hexagonal(value),
            7 => FigurateNumber::Heptagonal(value),
            8 => FigurateNumber::Octagonal(value),
            _ => unimplemented!(),
        }
    }

    fn value(self) -> u32 {
        match self {
            FigurateNumber::Triangle(x) => x,
            FigurateNumber::Square(x) => x,
            FigurateNumber::Pentagonal(x) => x,
            FigurateNumber::Hexagonal(x) => x,
            FigurateNumber::Heptagonal(x) => x,
            FigurateNumber::Octagonal(x) => x,
        }
    }

    fn get_type(self) -> mem::Discriminant<FigurateNumber> {
        mem::discriminant(&self)
    }
}

/// The ordered set of three 4-digit numbers: 8128, 2882, 8281, has three interesting properties.
///
/// 1. The set is cyclic, in that the last two digits of each number is the first two digits of the
/// 2. next number (including the last number with the first).
/// 3. Each polygonal type: triangle (P3,127=8128), square (P4,91=8281), and pentagonal (P5,44=2882),
///    is represented by a different number in the set.
/// This is the only set of 4-digit numbers with this property.
///
/// Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal
/// type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a
/// different number in the set.
fn main() {
    let figurate_numbers = SIDES
        .iter()
        .flat_map(|&s| {
            PolygonalNumberIterator::new(s)
                .take_while(|&p| p < MAX_NUMBER)
                .filter_map(move |p| {
                    if p > MIN_NUMBER {
                        Some(FigurateNumber::new(s, p))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<FigurateNumber>>();

    // There should only be one cycle.
    let cycles = generate_cycles(&figurate_numbers, SIDES.len());
    let answer: u32 = cycles.first().unwrap().iter().map(|f| f.value()).sum();
    println!("The answer is: {}", answer);
}

fn link_numbers(a: FigurateNumber, b: FigurateNumber) -> bool {
    a.value() % 100 == b.value() / 100
}

// Filter out the same variants and only retain those sharing the same two digits.
fn next_numbers(a: FigurateNumber, b: &[FigurateNumber]) -> impl Iterator<Item = &FigurateNumber> {
    let filter_a =
        move |b: &&FigurateNumber| -> bool { link_numbers(a, **b) && a.get_type() != b.get_type() };

    b.iter().filter(filter_a)
}

fn generate_cycles(numbers: &[FigurateNumber], length: usize) -> Vec<Vec<FigurateNumber>> {
    // Start with the triangle numbers.
    let mut l = 1;
    let mut cycles = numbers
        .iter()
        .filter(|n| n.get_type() == FigurateNumber::Triangle(0).get_type())
        .map(|&n| vec![n])
        .collect::<Vec<Vec<FigurateNumber>>>();

    // Then add numbers progressively until we reach the required length of the cycle, taking
    // care to eliminate those sharing a type of figurate number or a value.
    while l < length {
        l += 1;
        cycles = cycles
            .iter()
            .flat_map(|cycle| {
                next_numbers(*cycle.last().unwrap(), numbers).filter_map(move |&b| {
                    let mut cycle = cycle.clone();
                    if cycle
                        .iter()
                        .all(|x| x.get_type() != b.get_type() && x.value() != b.value())
                    {
                        cycle.push(b);
                        Some(cycle)
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<Vec<FigurateNumber>>>();
    }

    // Finally, filter out those that are not cyclical.
    cycles.retain(|cycle| link_numbers(*cycle.last().unwrap(), *cycle.first().unwrap()));
    cycles
}
