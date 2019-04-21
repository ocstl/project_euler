const COINS: [usize; 8] = [200, 100, 50, 20, 10, 5, 2, 1];
const INPUT: usize = 200;

fn main() {
    let answer = change(INPUT, &COINS);
    println!("Answer: {}", answer);
}

fn change(amount: usize, coins: &[usize]) -> usize {
    /* Return early when impossible. */
    if amount != 0 && coins.is_empty() {
        0
    } else {
        let coin = coins[0];
        let max_nbr = amount / coin;

        (0..=max_nbr)
            .map(|x| {
                let new_amount = amount - x * coin;
                if new_amount == 0 {
                    1
                } else {
                    change(new_amount, &coins[1..])
                }
            })
            .sum()
    }
}
