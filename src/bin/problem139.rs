use project_euler::pythagorean_triple::PrimitivePythagoreanTriple;

const INPUT: u64 = 100_000_000;

/// Let (a, b, c) represent the three sides of a right angle triangle with
/// integral length sides. It is possible to place four such triangles together
/// to form a square with length c.
///
/// Given that the perimeter of the right triangle is less than one-hundred
/// million, how many Pythagorean triangles would allow such a tiling to take
/// place?
fn main() {
    let mut primitives = vec![PrimitivePythagoreanTriple::default()];
    let mut count = 0;

    while let Some(primitive) = primitives.pop() {
        let triple = primitive.to_pythagorean_triple();
        if triple.perimeter() >= INPUT {
            continue;
        }

        primitives.extend(primitive.next_primitives());

        // We can tile if c is a multiple of the difference between a and b,
        // where the sides a, b, c follow a < b < c.
        let mut sides = triple.sides();
        sides.sort();

        if sides[2] % (sides[1] - sides[0]) == 0 {
            count += triple
                .multiples()
                .take_while(|t| t.perimeter() < INPUT)
                .count();
        }
    }

    println!("The answer is: {}", count);
}
