use primal::is_prime;

const INPUT: usize = 2000;

/// A hexagonal tile with number 1 is surrounded by a ring of six hexagonal
/// tiles, starting at "12 o'clock" and numbering the tiles 2 to 7 in an
/// anti-clockwise direction.
///
/// New rings are added in the same fashion, with the next rings being numbered
/// 8 to 19, 20 to 37, 38 to 61, and so on.
///
/// By finding the difference between tile n and each of its six neighbours we
/// shall define PD(n) to be the number of those differences which are prime.
///
/// For example, working clockwise around tile 8 the differences are 12, 29, 11,
/// 6, 1, and 13. SoPD(8) = 3.
///
/// In the same way, the differences around tile 17 are 1, 17, 16, 1, 11, and
/// 10, hence PD(17) = 2.
///
/// It can be shown that the maximum value of PD(n) is 3.
///
/// If all of the tiles for which PD(n) = 3 are listed in ascending order to
/// form a sequence, the 10th tile would be 271.
///
/// Find the 2000th tile in this sequence.
fn main() {
    // Define corners as the six straight lines from the center (1), and the
    // sides as all the others (the smallest side being 9).
    //
    // Ignoring the exception that is the side preceding the north corner,
    // a side N is preceded by N-1 and followed by N+1, and 1 is not a prime.
    // The inner ring forms one pair, with an even and an odd number, so only
    // one prime can be found. Same for the outer ring. So, a side cannot have
    // more than 2 differences that are prime.
    //
    // Ignoring the exception of the north corner, a corner N is preceded by
    // N-1 and followed by N+1, and 1 is not a prime. That leaves 1 number from
    // the inner ring, and 3 from the outer ring. If the difference with the
    // next corner (outer ring) is a prime, then the differences with both
    // other members of the outer ring are not, so we can't have 3 prime
    // differences.
    // Now, moving from the inner ring to outer ring corner adds an even number
    // to the inner ring; so, if the inner ring was even, the outer ring is
    // even, and odd also remains odd. Which means that the two neighbours of
    // the further corners have the reverse parity; N should thus be checked
    // against a (odd, even, even) or a (even, odd, odd) triplet, and neither
    // can yield 3 primes.
    //
    // The only remaining exceptions are the north corner and its south-east
    // neighbour.
    // The north corner (ring r) will have neighbours (counter-clockwise):
    // - N + 6r
    // - N + 6r + 1
    // - N + 1
    // - N - 6 (r - 1)
    // - N + 6r - 1
    // - N + 12r + 5
    //
    // The south-east neighbour of the north corner will have neighbours:
    // - N + 6 (r + 1)
    // - N - 6r + 1
    // - N - 12r + 7
    // - N - 6r
    // - N - 1
    // - N + 6 (r + 1) - 1
    //
    // Since we're only interested in differences that are prime, we can ignore
    // the N and concentrate on the possible prime differences. Thus, we can
    // eliminate the multiples of 6. So, for any north corner, we need to check:
    // - 6r + 1
    // - 6r - 1
    // - 12r + 5
    //
    // For the south-east neighbour:
    // - 6r - 1
    // - 12r - 7
    // - 6 (r + 1) - 1
    //
    // The first ring is a bit of an oddball, but we still manage to catch 2.

    let check_north_corner = |r| is_prime(6 * r + 1) && is_prime(6 * r - 1) && is_prime(12 * r + 5);
    let check_se_neighbour = |r| is_prime(6 * r - 1) && is_prime(12 * r - 7) && is_prime(6 * r + 5);

    // Start at ring 2 to avoid underflow and the problematic 7. Offset by 1 for
    // `nth`'s 0-indexing.
    let answer = [1_u64, 2]
        .into_iter()
        .cloned()
        .chain((2_u64..).flat_map(|r| {
            match (check_north_corner(r), check_se_neighbour(r)) {
                (true, true) => Some(2 + 3 * r * (r - 1))
                    .into_iter()
                    .chain(Some(1 + 3 * r * (r + 1)).into_iter()),
                (true, false) => Some(2 + 3 * r * (r - 1))
                    .into_iter()
                    .chain(None.into_iter()),
                (false, true) => Some(1 + 3 * r * (r + 1))
                    .into_iter()
                    .chain(None.into_iter()),
                (false, false) => None.into_iter().chain(None.into_iter()),
            }
        }))
        .nth(INPUT - 1)
        .unwrap();

    println!("The answer is: {:?}", answer);
}
