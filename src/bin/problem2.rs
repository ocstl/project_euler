use project_euler::fibonacci::Fibonacci;

const LIMIT: u32 = 40;

fn main() {
    let answer: u32 = Fibonacci::<u32>::new().take_while(|&x| x < LIMIT)
        .filter(|x| x % 2 == 0)
        .sum();

    println!("Answer: {}", answer);
}
