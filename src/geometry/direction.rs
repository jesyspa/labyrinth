use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West]
            .iter()
            .copied()
    }
}
