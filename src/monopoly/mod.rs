use rand::{thread_rng, Rng};

const CHANCE_SIZE: u8 = 16;
const COMMUNITY_CHEST_SIZE: u8 = 16;
const DICE_SIZE: usize = 4;
const DOUBLE_ROLLS_JAIL: usize = 3;

fn roll() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, DICE_SIZE) + 1
}

#[derive(Clone, Copy, PartialEq)]
#[repr(usize)]
enum Square {
    Go = 0,
    Property,
    CommunityChest,
    Tax,
    Railroad,
    Chance,
    Jail = 10,
    Utility,
    FreeParking,
    GoToJail = 30,
}

#[rustfmt::skip]
const BOARD: [Square; 40] = [
    Square::Go, Square::Property, Square::CommunityChest, Square::Property, Square::Tax,
    Square::Railroad, Square::Property, Square::Chance,  Square::Property, Square::Property,
    Square::Jail, Square::Property, Square::Utility, Square::Property, Square::Property,
    Square::Railroad, Square::Property, Square::CommunityChest, Square::Property, Square::Property,
    Square::FreeParking, Square::Property, Square::Chance, Square::Property, Square::Property,
    Square::Railroad, Square::Property, Square::Property, Square::Utility, Square::Property,
    Square::GoToJail, Square::Property, Square::Property, Square::CommunityChest, Square::Property,
    Square::Railroad, Square::Chance, Square::Property, Square::Tax, Square::Property
];

/// This not being a real monopoly game, our representation of a monopoly game is somewhat
/// simpler, as we do not need to represent the whole set. Also, given that there might be any
/// number of players, the steady state representation is 1/16 probability of any card.
/// Using a `u8` representation simplifies casting and the generation of a random card.
#[repr(usize)]
enum CommunityChest {
    Go,
    Jail,
    Other,
}

impl CommunityChest {
    fn draw() -> Self {
        match thread_rng().gen_range(0, COMMUNITY_CHEST_SIZE) {
            0 => CommunityChest::Go,
            1 => CommunityChest::Jail,
            _ => CommunityChest::Other,
        }
    }
}

#[repr(usize)]
enum Chance {
    Go,
    Jail,
    C1,
    E3,
    H2,
    R1,
    NextRailroad,
    NextUtility,
    Back3Squares,
    Other,
}

impl Chance {
    fn draw() -> Self {
        match thread_rng().gen_range(0, CHANCE_SIZE) {
            0 => Chance::Go,
            1 => Chance::Jail,
            2 => Chance::C1,
            3 => Chance::E3,
            4 => Chance::H2,
            5 => Chance::R1,
            6 | 7 => Chance::NextRailroad,
            8 => Chance::NextUtility,
            9 => Chance::Back3Squares,
            _ => Chance::Other,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Player {
    double_rolls: usize,
    location: usize,
}

impl Player {
    pub fn new() -> Self {
        Player {
            double_rolls: 0,
            location: Square::Go as usize,
        }
    }

    pub fn location(&self) -> usize {
        self.location
    }

    fn go_to_go(&mut self) {
        self.location = Square::Go as usize;
    }

    fn go_to_jail(&mut self) {
        self.location = Square::Jail as usize;
    }

    fn turn(&mut self) {
        let a = roll();
        let b = roll();

        // Count the number of consecutive double rolls.
        if a == b {
            self.double_rolls += 1;
        } else {
            self.double_rolls = 0;
        }

        // Too many double rolls lead to jail.
        if self.double_rolls == DOUBLE_ROLLS_JAIL {
            self.go_to_jail();
            self.double_rolls = 0;
            return;
        }

        self.location = (self.location + a + b) % BOARD.len();
        let square = BOARD[self.location];

        if square == Square::GoToJail {
            self.go_to_jail();
            return;
        }

        // Since Chance can move us back unto a CommunityChest, we deal with it first. Sorry
        // about the magic number.
        if square == Square::Chance {
            match Chance::draw() {
                Chance::Go => self.go_to_go(),
                Chance::Jail => self.go_to_jail(),
                Chance::C1 => self.location = 11,
                Chance::E3 => self.location = 24,
                Chance::H2 => self.location = 39,
                Chance::R1 => self.location = 5,
                Chance::NextRailroad => {
                    while BOARD[self.location] != Square::Railroad {
                        self.location = (self.location + 1) % BOARD.len();
                    }
                }
                Chance::NextUtility => {
                    while BOARD[self.location] != Square::Utility {
                        self.location = (self.location + 1) % BOARD.len();
                    }
                }
                Chance::Back3Squares => self.location -= 3,
                Chance::Other => (),
            }
        }

        let square = BOARD[self.location];

        // Deal with the possible CommunityChest.
        if square == Square::CommunityChest {
            match CommunityChest::draw() {
                CommunityChest::Go => self.go_to_go(),
                CommunityChest::Jail => self.go_to_jail(),
                CommunityChest::Other => (),
            }
        }
    }
}

impl Iterator for Player {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.turn();
        Some(*self)
    }
}
