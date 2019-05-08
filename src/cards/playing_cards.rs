use core::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn successor(&self) -> Self {
        match self {
            Value::Two => Value::Three,
            Value::Three => Value::Four,
            Value::Four => Value::Five,
            Value::Five => Value::Six,
            Value::Six => Value::Seven,
            Value::Seven => Value::Eight,
            Value::Eight => Value::Nine,
            Value::Nine => Value::Ten,
            Value::Ten => Value::Jack,
            Value::Jack => Value::Queen,
            Value::Queen => Value::King,
            Value::King => Value::Ace,
            Value::Ace => Value::Two,
        }
    }

    pub fn predecessor(&self) -> Self {
        match self {
            Value::Two => Value::Ace,
            Value::Three => Value::Two,
            Value::Four => Value::Three,
            Value::Five => Value::Four,
            Value::Six => Value::Five,
            Value::Seven => Value::Six,
            Value::Eight => Value::Seven,
            Value::Nine => Value::Eight,
            Value::Ten => Value::Nine,
            Value::Jack => Value::Ten,
            Value::Queen => Value::Jack,
            Value::King => Value::Queen,
            Value::Ace => Value::King,
        }
    }
}

impl From<char> for Value {
    fn from(input: char) -> Self {
        match input {
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' => Value::Ace,
            x => panic!("Unrecognized card value: {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl From<char> for Suit {
    fn from(input: char) -> Self {
        match input {
            'D' => Suit::Diamonds,
            'C' => Suit::Clubs,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            x => panic!("Unrecognized suit: {}", x),
        }
    }
}

// Implement PartialOrd and Ord manually to impose no ordering.
impl PartialOrd for Suit {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for Suit {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl Card {
    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}

impl From<&str> for Card {
    fn from(input: &str) -> Self {
        let mut iter = input.chars();
        Card {
            value: Value::from(iter.next().unwrap()),
            suit: Suit::from(iter.next().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! card {
        ($x:expr) => {
            Card::from($x)
        };
    }

    #[test]
    fn test_equality() {
        assert_eq!(card!("5H"), card!("5H"));
    }

    #[test]
    fn test_inequality_value() {
        assert_ne!(card!("5H"), card!("6H"));
    }

    #[test]
    fn test_inequality_suit() {
        assert_ne!(card!("5H"), card!("5S"));
    }

    #[test]
    fn test_ordering_value() {
        assert!(card!("5H") < card!("6H"));
    }

    #[test]
    fn test_ordering_suit() {
        assert_eq!(card!("5H").partial_cmp(&card!("5D")), Some(Ordering::Equal));
        assert_eq!(card!("5H").cmp(&card!("5D")), Ordering::Equal);
    }
}
