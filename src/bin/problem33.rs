use num::Rational;

fn main() {
    /* We are dealing with two digits numerators and denominators, thus we can hard code a bit. */
    let digit_cancelling_fraction = |numerator, denominator| -> bool {
        let ratio = Rational::new(numerator, denominator);

        /* Eliminate trivial fractions. */
        if numerator % 10 == 0 && denominator % 10 == 0 {
            false
        } else {
            ((numerator / 10) == (denominator / 10) && denominator % 10 != 0 &&
                    Rational::new(numerator % 10, denominator % 10) == ratio)
                || ((numerator / 10) == (denominator % 10) &&
                    Rational::new(numerator % 10, denominator / 10) == ratio)
                || ((numerator % 10) == (denominator / 10) && denominator % 10 != 0 &&
                    Rational::new(numerator / 10, denominator % 10) == ratio)
                || ((numerator % 10) == (denominator % 10) &&
                    Rational::new(numerator / 10, denominator / 10) == ratio)
        }
    };

    let fractions: Vec<Rational> = (11..100).flat_map(move |denominator: isize| (10..denominator)
        .filter(move |&numerator| digit_cancelling_fraction(numerator, denominator))
        .map(move |numerator| Rational::new(numerator, denominator)))
        .collect();

    let answer = *fractions.iter().product::<Rational>().denom();

    println!("Answer: {}", answer);
}