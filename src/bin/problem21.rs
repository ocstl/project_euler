use project_euler::aliquot_sum_fn;

const INPUT: usize = 10000;

aliquot_sum_fn!(INPUT);

/// Evaluate the sum of all the amicable numbers under 10000.
fn main() {
    let answer: usize = (2..INPUT)
        .filter(|&x| {
            let y = aliquot_sum(x);
            y != x && x == aliquot_sum(y)
        })
        .sum();

    println!("Answer: {}", answer);
}
