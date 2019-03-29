use project_euler::collatz::CollatzSequence;

const INPUT: usize = 1_000_000;

fn main() {
    /* Use 0 to pad the Vec, so the indices are aligned. */
    let mut cache: Vec<usize> = Vec::with_capacity(INPUT + 1);
    cache.push(0);
    cache.push(1);

    for x in 2..INPUT {
        let length = CollatzSequence::new(x).enumerate()
            .find_map(|(count, current)| {
                if let Some(i) = cache.get(current) {
                    Some(1 + i + count)
                } else {
                    None
                }
            });
        cache.push(length.unwrap());
    }

    let answer = cache.iter()
        .enumerate()
        .max_by_key(|(_idx, &v)| v)
        .unwrap();

    println!("Answer: {}", answer.0);
}