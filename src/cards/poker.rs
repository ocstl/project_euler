use super::cards::{Card, Value};

type FiveCards = [Card; 5];

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
// Add values at the beginning to implement PartialOrd and Ord easily.
pub enum Hand {
    HighCard(FiveCards),
    OnePair(Value, FiveCards),
    TwoPairs(Value, Value, FiveCards),
    ThreeOfAKind(Value, FiveCards),
    Straight(FiveCards),
    Flush(FiveCards),
    FullHouse(Value, Value, FiveCards),
    FourOfAKind(Value, FiveCards),
    StraightFlush(FiveCards),
    RoyalFlush(FiveCards),
}

impl Hand {
    fn is_flush(cards: &FiveCards) -> bool {
        cards[1..].iter().all(|card| card.suit() == cards[0].suit())
    }

    fn is_straight(cards: &FiveCards) -> bool {
        cards
            .windows(2)
            .all(|c| c[0].value().predecessor() == *c[1].value())
    }

    fn four_of_a_kind(cards: &FiveCards) -> Option<Value> {
        cards
            .iter()
            .find(|&card| cards.iter().filter(|&c| c.value() == card.value()).count() == 4)
            .and_then(|card| Some(card.value().clone()))
    }

    fn three_of_a_kind(cards: &FiveCards) -> Option<Value> {
        cards
            .iter()
            .find(|&card| cards.iter().filter(|&c| c.value() == card.value()).count() == 3)
            .and_then(|card| Some(card.value().clone()))
    }

    fn pairs(cards: &FiveCards) -> (Option<Value>, Option<Value>) {
        let p1 = cards
            .iter()
            .find(|&card| cards.iter().filter(|&c| c.value() == card.value()).count() == 2)
            .and_then(|card| Some(card.value().clone()));

        if let Some(v) = p1.clone() {
            let p2 = cards
                .iter()
                .find(|&card| {
                    *card.value() != v
                        && cards.iter().filter(|&c| c.value() == card.value()).count() == 2
                })
                .and_then(|card| Some(card.value().clone()));

            (p1, p2)
        } else {
            (None, None)
        }
    }
}

impl From<[&str; 5]> for Hand {
    fn from(input: [&str; 5]) -> Self {
        let mut cards: FiveCards = [
            Card::from(input[0]),
            Card::from(input[1]),
            Card::from(input[2]),
            Card::from(input[3]),
            Card::from(input[4]),
        ];
        cards.sort_by(|a, b| a.cmp(b).reverse());

        let straight = Hand::is_straight(&cards);
        let flush = Hand::is_flush(&cards);
        let four_of_a_kind = Hand::four_of_a_kind(&cards);
        let three_of_a_kind = Hand::three_of_a_kind(&cards);
        let (p1, p2) = Hand::pairs(&cards);
        let ace_high = cards[0].value() == &Value::Ace;

        match (
            straight,
            flush,
            four_of_a_kind,
            three_of_a_kind,
            p1,
            p2,
            ace_high,
        ) {
            (true, true, .., true) => Hand::RoyalFlush(cards),
            (true, true, .., false) => Hand::StraightFlush(cards),
            (_, _, Some(v), ..) => Hand::FourOfAKind(v, cards),
            (_, _, _, Some(v), Some(w), ..) => Hand::FullHouse(v, w, cards),
            (_, true, ..) => Hand::Flush(cards),
            (true, ..) => Hand::Straight(cards),
            (_, _, _, Some(v), ..) => Hand::ThreeOfAKind(v, cards),
            (_, _, _, _, Some(v), Some(w), ..) => Hand::TwoPairs(v, w, cards),
            (_, _, _, _, Some(v), ..) => Hand::OnePair(v, cards),
            _ => Hand::HighCard(cards),
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

    macro_rules! hand {
        ($x:expr) => {
            Hand::from($x)
        };
    }

    macro_rules! royal_straight_flush {
        () => {
            Hand::RoyalFlush([
                card!("AD"),
                card!("KD"),
                card!("QD"),
                card!("JD"),
                card!("TD"),
            ]);
        };
    }

    macro_rules! straight_flush {
        () => {
            Hand::StraightFlush([
                card!("KD"),
                card!("QD"),
                card!("JD"),
                card!("TD"),
                card!("9D"),
            ]);
        };
    }

    macro_rules! four_of_a_kind {
        () => {
            Hand::FourOfAKind(
                Value::Two,
                [
                    card!("AS"),
                    card!("2D"),
                    card!("2C"),
                    card!("2H"),
                    card!("2S"),
                ],
            );
        };
    }

    macro_rules! full_house {
        () => {
            Hand::FullHouse(
                Value::Three,
                Value::Two,
                [
                    card!("3D"),
                    card!("3C"),
                    card!("3H"),
                    card!("2D"),
                    card!("2C"),
                ],
            );
        };
    }

    macro_rules! flush {
        () => {
            Hand::Flush([
                card!("8D"),
                card!("5D"),
                card!("4D"),
                card!("3D"),
                card!("2D"),
            ]);
        };
    }

    macro_rules! straight {
        () => {
            Hand::Straight([
                card!("6C"),
                card!("5D"),
                card!("4D"),
                card!("3D"),
                card!("2D"),
            ]);
        };
    }

    macro_rules! three_of_a_kind {
        () => {
            Hand::ThreeOfAKind(
                Value::Two,
                [
                    card!("AS"),
                    card!("KD"),
                    card!("2C"),
                    card!("2H"),
                    card!("2S"),
                ],
            );
        };
    }

    macro_rules! two_pairs {
        () => {
            Hand::TwoPairs(
                Value::Three,
                Value::Two,
                [
                    card!("4D"),
                    card!("3C"),
                    card!("3H"),
                    card!("2D"),
                    card!("2C"),
                ],
            );
        };
    }

    macro_rules! one_pair {
        () => {
            Hand::OnePair(
                Value::Three,
                [
                    card!("AC"),
                    card!("4D"),
                    card!("3C"),
                    card!("3H"),
                    card!("2D"),
                ],
            );
        };
    }

    macro_rules! high_card {
        () => {
            Hand::HighCard([
                card!("AC"),
                card!("6C"),
                card!("4D"),
                card!("3H"),
                card!("2D"),
            ]);
        };
    }

    #[test]
    fn test_royal_straight_flush() {
        let actual = hand!(["TD", "JD", "QD", "KD", "AD"]);
        let expected = royal_straight_flush!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_royal_beats_all() {
        assert!(royal_straight_flush!() > straight_flush!());
        assert!(royal_straight_flush!() > four_of_a_kind!());
        assert!(royal_straight_flush!() > full_house!());
        assert!(royal_straight_flush!() > flush!());
        assert!(royal_straight_flush!() > straight!());
        assert!(royal_straight_flush!() > three_of_a_kind!());
        assert!(royal_straight_flush!() > two_pairs!());
        assert!(royal_straight_flush!() > one_pair!());
        assert!(royal_straight_flush!() > high_card!());
    }

    #[test]
    fn test_straight_flush() {
        let actual = hand!(["TD", "JD", "QD", "KD", "9D"]);
        let expected = straight_flush!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn straight_flush_beats() {
        assert!(straight_flush!() > four_of_a_kind!());
        assert!(straight_flush!() > full_house!());
        assert!(straight_flush!() > flush!());
        assert!(straight_flush!() > straight!());
        assert!(straight_flush!() > three_of_a_kind!());
        assert!(straight_flush!() > two_pairs!());
        assert!(straight_flush!() > one_pair!());
        assert!(straight_flush!() > high_card!());
    }

    #[test]
    fn test_four_of_a_kind() {
        let actual = hand!(["2D", "2C", "2H", "2S", "AS"]);
        let expected = four_of_a_kind!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn four_of_a_kind_beats() {
        assert!(four_of_a_kind!() > full_house!());
        assert!(four_of_a_kind!() > flush!());
        assert!(four_of_a_kind!() > straight!());
        assert!(four_of_a_kind!() > three_of_a_kind!());
        assert!(four_of_a_kind!() > two_pairs!());
        assert!(four_of_a_kind!() > one_pair!());
        assert!(four_of_a_kind!() > high_card!());
    }

    #[test]
    fn test_full_house() {
        let actual = hand!(["2D", "3D", "3C", "3H", "2C"]);
        let expected = full_house!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn full_house_beats() {
        assert!(full_house!() > flush!());
        assert!(full_house!() > straight!());
        assert!(full_house!() > three_of_a_kind!());
        assert!(full_house!() > two_pairs!());
        assert!(full_house!() > one_pair!());
        assert!(full_house!() > high_card!());
    }

    #[test]
    fn test_flush() {
        let actual = hand!(["2D", "3D", "4D", "5D", "8D"]);
        let expected = flush!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn flush_beats() {
        assert!(flush!() > straight!());
        assert!(flush!() > three_of_a_kind!());
        assert!(flush!() > two_pairs!());
        assert!(flush!() > one_pair!());
        assert!(flush!() > high_card!());
    }

    #[test]
    fn test_straight() {
        let actual = hand!(["2D", "3D", "4D", "5D", "6C"]);
        let expected = straight!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn straight_beats() {
        assert!(straight!() > three_of_a_kind!());
        assert!(straight!() > two_pairs!());
        assert!(straight!() > one_pair!());
        assert!(straight!() > high_card!());
    }

    #[test]
    fn test_three_of_a_kind() {
        let actual = hand!(["KD", "2C", "2H", "2S", "AS"]);
        let expected = three_of_a_kind!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn three_of_a_kind_beats() {
        assert!(three_of_a_kind!() > two_pairs!());
        assert!(three_of_a_kind!() > one_pair!());
        assert!(three_of_a_kind!() > high_card!());
    }

    #[test]
    fn test_two_pairs() {
        let actual = hand!(["2D", "4D", "3C", "3H", "2C"]);
        let expected = two_pairs!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn two_pairs_beats() {
        assert!(two_pairs!() > one_pair!());
        assert!(two_pairs!() > high_card!());
    }

    #[test]
    fn test_one_pair() {
        let actual = hand!(["4D", "3C", "3H", "2D", "AC"]);
        let expected = one_pair!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_pair_beats() {
        assert!(one_pair!() > high_card!());
    }

    #[test]
    fn test_high_card() {
        let actual = hand!(["2D", "3H", "4D", "6C", "AC"]);
        let expected = high_card!();

        assert_eq!(actual, expected);
    }

    #[test]
    fn high_card_beats() {
        assert!(hand!(["2D", "3H", "4D", "6C", "AC"]) > hand!(["KC", "3H", "4D", "6C", "2D"]));
    }
}
