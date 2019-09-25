use num::integer::Roots;

const INPUT: usize = 1_000_000;

/// A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits
/// in the opposite corner. By travelling on the surfaces of the room the shortest "straight
/// line" distance from S to F is 10.
///
/// However, there are up to three "shortest" path candidates for any given cuboid and the
/// shortest route doesn't always have integer length.
///
/// It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer
/// dimensions, up to a maximum size of M by M by M, for which the shortest route has integer
/// length when M = 100. This is the least value of M for which the number of solutions first
/// exceeds two thousand; the number of solutions when M = 99 is 1975.
///
/// Find the least value of M such that the number of solutions first exceeds one million.
fn main() {
    let nbr_a = |a: u64| -> usize {
        (1..=a)
            .flat_map(|b| (1..=b).filter(move |&c| shortest_distance_is_integer(a, b, c)))
            .count()
    };

    let answer = (1..)
        .scan((0, 0), |state, a| {
            *state = (state.0 + nbr_a(a), a);
            Some(*state)
        })
        .find(|&(n, _)| n > INPUT)
        .unwrap()
        .1;

    println!("The answer is: {}", answer);
}

/// The shortest distance is given by the square root of (a^2 + (b + c)^2),
/// where b and c are the shortest dimensions. This will yield an integer only if it is a perfect
/// square.
fn shortest_distance_is_integer(a: u64, b: u64, c: u64) -> bool {
    let x = a * a + (b + c) * (b + c);
    let s = x.sqrt();
    x == s * s
}
