use std::iter::repeat;

const INPUT: usize = 1000;

fn main() {
    let triplet = (1..INPUT).flat_map(|a| repeat(a).zip(a+1..=INPUT-a))
        .filter_map(|(a, b)| {
            let c = INPUT - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                Some((a, b, c))
            } else {
                None
            }
        }).next();

    if let Some((a, b, c)) = triplet {
        let answer = a * b * c;
        println!("Answer: {}", answer);
    } else {
        println!("No answer.");
    }
}
