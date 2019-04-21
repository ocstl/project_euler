/* With a reduced fraction m/n, the period of length t begins after s terms, where:
    10^s == 10^(s+t) mod n.
*/

fn period_length(x: usize) -> usize {
    /* Products of the powers of 2 and 5 do not have a repeating cycle. */
    let mut x = x;
    while x % 2 == 0 {
        x /= 2;
    }

    while x % 5 == 0 {
        x /= 5;
    }

    if x == 1 {
        0
    } else {
        /* Otherwise, the length of the period is given by n in 10^n = 1 mod x. */
        let mut count = 0;
        let mut current = 1;

        while current != 1 || count == 0 {
            count += 1;
            current *= 10;
            current %= x;
        }

        count
    }
}

fn main() {
    let longest_cycle = (2..1000).max_by_key(|&x| period_length(x)).unwrap();

    println!("Answer: {}", longest_cycle);
}
