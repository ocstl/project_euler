use counter::Counter;
use project_euler::pythagorean_triple::PrimitivePythagoreanTriple;

const INPUT: u64 = 1_500_000;

/// It turns out that 12 cm is the smallest length of wire that can be bent to form an integer
/// sided right angle triangle in exactly one way, but there are many more examples.
///
/// Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one
/// integer sided right angle triangle be formed?
///
/// Integer sided right triangles are given by the Pythagorean triplets. We need only to generate
/// them all.
fn main() {
    // Since all primitives are unique, we only need the lengths of their multiples, so no need
    // to store the full triples.
    let mut counter: Counter<u64, u64> = Counter::new();
    let mut primitives = vec![PrimitivePythagoreanTriple::default()];

    while let Some(primitive) = primitives.pop() {
        let length = primitive.to_pythagorean_triple().perimeter();

        // Skip primitives that are too large to begin with, as their multiples won't count, and
        // the generated primitives will be even larger.
        if length <= INPUT {
            // Add all multiples that fit.
            let it = (1..)
                .map(|multiple| multiple * length)
                .take_while(|&n| n <= INPUT);

            counter.update(it);
            primitives.extend(primitive.next_primitives());
        }
    }

    let answer = counter.values().filter(|&&v| v == 1).count();
    println!("The answer is: {}", answer);
}
