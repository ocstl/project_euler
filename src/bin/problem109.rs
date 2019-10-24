use std::collections::HashSet;
use std::iter::once;

const INPUT: u32 = 100;
const REGIONS: [u32; 3] = [1, 2, 3];
const SECTIONS: [u32; 21] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 25,
];

/// In the game of darts a player throws three darts at a target board which is split into twenty
/// equal sized sections numbered one to twenty.
///
/// The score of a dart is determined by the number of the region that the dart lands in. A dart
/// landing outside the red/green outer ring scores zero. The black and cream regions inside this
/// ring represent single scores. However, the red/green outer ring and middle ring score double
/// and treble scores respectively.
///
/// At the centre of the board are two concentric circles called the bull region, or bulls-eye.
/// The outer bull is worth 25 points and the inner bull is a double, worth 50 points.
///
/// There are many variations of rules but in the most popular game the players will begin with a
/// score 301 or 501 and the first player to reduce their running total to zero is a winner.
/// However, it is normal to play a "doubles out" system, which means that the player must land a
/// double (including the double bulls-eye at the centre of the board) on their final dart to
/// win; any other dart that would reduce their running total to one or lower means the score for
/// that set of three darts is "bust".
///
/// When a player is able to finish on their current score it is called a "checkout" and the
/// highest checkout is 170: T20 T20 D25 (two treble 20s and double bull).
fn main() {
    let possible_shots = SECTIONS
        .iter()
        .flat_map(|&section| REGIONS.iter().map(move |&region| (section, region)))
        .filter(|&(section, region)| section != 25 || region != 3)
        .chain(once((0, 0)));

    let two_shots: HashSet<((u32, u32), (u32, u32))> = possible_shots
        .clone()
        .flat_map(|first_shot| {
            possible_shots.clone().map(move |shot| {
                if first_shot <= shot {
                    (first_shot, shot)
                } else {
                    (shot, first_shot)
                }
            })
        })
        .collect();

    let final_shot = SECTIONS.iter().map(|&section| section << 1);

    let answer = final_shot
        .flat_map(|final_shot| {
            two_shots
                .iter()
                .map(move |&(shot1, shot2)| final_shot + shot1.0 * shot1.1 + shot2.0 * shot2.1)
        })
        .filter(|&total| total < INPUT)
        .count();

    println!("The answer is: {}", answer);
}
