const LIMIT: u32 = 1000;

fn main() {
    let answer: u32 = (1..LIMIT).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("Answer: {}", answer);
}
