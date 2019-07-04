pub struct PolygonalNumberIterator {
    side: u32,
    current: u32,
}

impl PolygonalNumberIterator {
    pub fn new(side: u32) -> Self {
        PolygonalNumberIterator { side, current: 0 }
    }
}

impl Iterator for PolygonalNumberIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        Some((self.side - 2) * self.current * (self.current - 1) / 2 + self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_numbers() {
        let mut actual = PolygonalNumberIterator::new(3);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(3));
        assert_eq!(actual.next(), Some(6));
    }

    #[test]
    fn square_numbers() {
        let mut actual = PolygonalNumberIterator::new(4);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(4));
        assert_eq!(actual.next(), Some(9));
    }

    #[test]
    fn pentagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(5);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(5));
        assert_eq!(actual.next(), Some(12));
    }

    #[test]
    fn hexagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(6);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(6));
        assert_eq!(actual.next(), Some(15));
    }
    #[test]
    fn heptagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(7);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(7));
        assert_eq!(actual.next(), Some(18));
    }
    #[test]
    fn octagonal_numbers() {
        let mut actual = PolygonalNumberIterator::new(8);

        assert_eq!(actual.next(), Some(1));
        assert_eq!(actual.next(), Some(8));
        assert_eq!(actual.next(), Some(21));
    }
}
