const INPUT: usize = 1001;

fn main() {
    /* Starting after the first step (2 for 1 to 3, 4 for 1 to 5, etc.), we increment the step by
    8 at every step. */
    let diagonal = move |init: usize| {
        (0..).map(move |x| init + 8 * x).scan(1, |state, x| {
            *state += x;
            Some(*state)
        })
    };

    /* Starting steps for the lower right, lower left, upper left and upper right diagonals. */
    let starting_steps = [2, 4, 6, 8];

    /* Take half of the full dimension (half diagonals). Don't forget to add the 1 at the center. */
    let answer: usize = starting_steps
        .iter()
        .flat_map(|&init| diagonal(init).take((INPUT - 1) / 2))
        .sum::<usize>()
        + 1;

    println!("Answer: {}", answer);
}
