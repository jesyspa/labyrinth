use super::Direction;
use std::ops::{Add, Mul};

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn within(self, other: Point) -> bool {
        self.0 < other.0 && self.1 < other.1
    }

    pub fn bounded_by(self, other: Point) -> bool {
        self.0 <= other.0 && self.1 <= other.1
    }

    pub fn transform(self, width: usize, direction: Direction) -> Self {
        match direction {
            Direction::North => Point(width - self.0 - 1, width - self.1 - 1),
            Direction::East => Point(width - self.1 - 1, self.0),
            Direction::South => self,
            Direction::West => Point(self.1, width - self.0 - 1),
        }
    }
}

impl Mul<Point> for usize {
    type Output = Point;
    fn mul(self, p: Point) -> Point {
        Point(self * p.0, self * p.1)
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Offset(pub usize, pub usize);

impl Add for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Offset {
        Offset(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Offset> for Point {
    type Output = Point;
    fn add(self, rhs: Offset) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Transform {
    width: usize,
    direction: Direction,
}

impl Transform {
    pub fn new(width: usize, direction: Direction) -> Self {
        Transform { width, direction }
    }
}

impl Mul<Point> for Transform {
    type Output = Point;
    fn mul(self, p: Point) -> Point {
        p.transform(self.width, self.direction)
    }
}

#[cfg(test)]
mod test {
    use super::Point;
    use crate::geometry::Direction;

    #[test]
    fn within() {
        assert!(Point(5, 3).within(Point(6, 4)));
        assert!(!Point(5, 3).within(Point(6, 3)));
        assert!(!Point(5, 3).within(Point(5, 4)));
        assert!(!Point(5, 3).within(Point(5, 3)));
    }

    #[test]
    fn bounded_by() {
        assert!(Point(5, 3).bounded_by(Point(6, 4)));
        assert!(Point(5, 3).bounded_by(Point(5, 3)));
        assert!(!Point(5, 3).within(Point(4, 3)));
        assert!(!Point(5, 3).within(Point(5, 2)));
        assert!(!Point(5, 3).within(Point(4, 2)));
    }

    #[test]
    fn transform() {
        assert_eq!(Point(0, 0).transform(4, Direction::North), Point(3, 3));
        assert_eq!(Point(0, 0).transform(4, Direction::East), Point(3, 0));
        assert_eq!(Point(0, 0).transform(4, Direction::South), Point(0, 0));
        assert_eq!(Point(0, 0).transform(4, Direction::West), Point(0, 3));

        assert_eq!(Point(1, 2).transform(4, Direction::North), Point(2, 1));
        assert_eq!(Point(1, 2).transform(4, Direction::East), Point(1, 1));
        assert_eq!(Point(1, 2).transform(4, Direction::South), Point(1, 2));
        assert_eq!(Point(1, 2).transform(4, Direction::West), Point(2, 2));
    }
}