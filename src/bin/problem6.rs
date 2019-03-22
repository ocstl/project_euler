const INPUT: usize = 100;

fn main() {
    let sum_squares: usize = (1..=INPUT).map(|x| x.pow(2)).sum();
    let square_sum: usize = (1..=INPUT).sum::<usize>().pow(2);

    let answer = square_sum - sum_squares;
    println!("Answer: {}", answer);
}
