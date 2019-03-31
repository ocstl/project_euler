use project_euler::num2word::num2word;

const INPUT: usize = 1000;

fn main() {
    let answer: usize = (1..=INPUT)
        .map(|x|
            num2word(x).chars().filter(|c| c.is_ascii_alphabetic()).count())
        .sum();

    println!("Answer: {}", answer);
}