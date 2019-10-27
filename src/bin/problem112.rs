use radixal::IntoDigits;

const TARGET: u64 = 99;
const BATCH: u64 = 100;

fn is_bouncy(n: u64) -> bool {
    let digits: Vec<u64> = n.into_decimal_digits().collect();
    digits.windows(2).any(|window| window[0] > window[1])
        && digits.windows(2).any(|window| window[0] < window[1])
}

/// Working from left-to-right if no digit is exceeded by the digit to its left it is called an
/// increasing number; for example, 134468.
///
/// Similarly if no digit is exceeded by the digit to its right it is called a decreasing number;
/// for example, 66420.
///
/// We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number;
/// for example, 155349.
///
/// Clearly there cannot be any bouncy numbers below one-hundred, but just over half of the
/// numbers below one-thousand (525) are bouncy. In fact, the least number for which the
/// proportion of bouncy numbers first reaches 50% is 538.
///
/// Surprisingly, bouncy numbers become more and more common and by the time we reach 21780 the
/// proportion of bouncy numbers is equal to 90%.
///
/// Find the least number for which the proportion of bouncy numbers is exactly 99%.
fn main() {
    // For the proportion to be exactly equal to 99%, we need to have n % 100 == 0 numbers. Start
    // at 100, since no number below that can be bouncy.
    let mut bouncies = 0;
    let mut answer = 0;
    for number in 100.. {
        if is_bouncy(number) {
            bouncies += 1;
        }

        if number % BATCH == 0 && number / BATCH * TARGET == bouncies {
            answer = number;
            break;
        }
    }

    println!("The answer is: {}", answer);
}
