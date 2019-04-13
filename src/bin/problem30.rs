const INPUT: u32 = 5;

fn main() {
    let sum_powers = |mut x: usize| {
        let mut sum = 0;
        while x > 0 {
            sum += (x % 10).pow(INPUT);
            x /= 10;
        }
        sum
    };

    /* Find an upper bound to limit the search. */
    let mut upper_bound = 9;
    while sum_powers(upper_bound) > upper_bound {
        upper_bound *= 10;
        upper_bound += 9;
    }

    let answer: usize = (2..upper_bound)
        .filter(|&x| x == sum_powers(x))
        .sum();

    println!("Answer: {}", answer);
}