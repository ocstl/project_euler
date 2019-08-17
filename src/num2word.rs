const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

/*
const SCALES: [&str; 5] = [
    "",
    "million",
    "billion",
    "trillion",
    "quadrillion",
];
*/

/* Using the British spelling. */
pub fn num2word(number: usize) -> String {
    if number == 0 {
        "zero".to_string()
    } else {
        match number {
            0..=9 => ONES[number].to_string(),
            10..=19 => TEENS[number - 10].to_string(),
            20..=99 => format!("{}-{}", TENS[number / 10], ONES[number % 10]),
            100..=999 => format!(
                "{} hundred and {}",
                ONES[number / 100],
                num2word(number % 100)
            ),
            1000..=999_999 => format!(
                "{} thousand {}",
                num2word(number / 1000),
                num2word(number % 1000)
            ),
            _ => panic!(),
        }
        .trim_end_matches(" zero")
        .trim_end_matches(" and")
        .trim_end_matches("-zero")
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($x:expr, $y:expr) => {
            assert_eq!(&num2word($x), $y);
        };
    }

    #[test]
    fn one() {
        test!(1, "one");
    }

    #[test]
    fn eleven() {
        test!(11, "eleven");
    }

    #[test]
    fn one_hundred() {
        test!(100, "one hundred");
    }

    #[test]
    fn two_hundred() {
        test!(200, "two hundred");
    }

    #[test]
    fn one_hundred_and_fifteen() {
        test!(115, "one hundred and fifteen");
    }

    #[test]
    fn one_hundred_twenty() {
        test!(120, "one hundred and twenty");
    }

    #[test]
    fn one_thousand() {
        test!(1000, "one thousand");
    }

    #[test]
    fn one_thousand_one_hundred_and_forty_two() {
        test!(1142, "one thousand one hundred and forty-two");
    }

    #[test]
    fn one_hundred_thousand() {
        test!(100_000, "one hundred thousand");
    }
}
