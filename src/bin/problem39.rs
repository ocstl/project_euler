const INPUT: usize = 1000;

fn right_triangles(perimeter: usize) -> impl Iterator<Item = (usize, usize, usize)> {
    (1..=perimeter / 4).flat_map(move |a| {
        (a..=perimeter / 2).filter_map(move |b| {
            let c = perimeter - a - b;
            if c * c == a * a + b * b {
                Some((a, b, c))
            } else {
                None
            }
        })
    })
}

fn main() {
    let answer = (1..=INPUT)
        .max_by_key(|&perimeter| right_triangles(perimeter).count())
        .unwrap_or(0);

    println!("Answer: {}", answer);
}
