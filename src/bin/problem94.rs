use num::integer::Roots;
use std::iter::once;

const INPUT: u64 = 1_000_000_000;

/// It is easily proved that no equilateral triangle exists with integral length sides and
/// integral area. However, the almost equilateral triangle 5-5-6 has an area of 12 square units.
///
/// We shall define an almost equilateral triangle to be a triangle for which two sides are equal
/// and the third differs by no more than one unit.
///
/// Find the sum of the perimeters of all almost equilateral triangles with integral side lengths
/// and area and whose perimeters do not exceed one billion (1,000,000,000).
fn main() {
    let answer: u64 = (2..)
        .flat_map(|a| once((a, a - 1)).chain(once((a, a + 1))))
        .take_while(|&(a, c)| perimeter(a, c) <= INPUT)
        .filter(|&(a, c)| integral_area(a, c))
        .map(|(a, c)| perimeter(a, c))
        .sum();

    println!("The answer is: {}", answer);
}

fn perimeter(a: u64, c: u64) -> u64 {
    2 * a + c
}

// Check whether the area is an integer.
// Using Heron's formula, the area is given by c * sqrt( a^2 - (c / 2)^2 ) where s is the
// semi-perimeter.
fn integral_area(a: u64, c: u64) -> bool {
    // `c``has to be even, otherwise the square root is over x.75, which is not an integer.
    if c & 1 == 1 {
        return false;
    }

    let tmp = a * a - (c >> 1) * (c >> 1);
    tmp.sqrt().pow(2) == tmp
}
