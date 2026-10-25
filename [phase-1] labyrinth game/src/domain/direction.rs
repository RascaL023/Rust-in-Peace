pub enum Direction {
    North, // UP
    South, // DOWN
    West,  // LEFT
    East,  // RIGHT
}

impl Direction {
    pub fn vector_from(&self) -> (i8, i8) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
            Self::East => (0, 1),
        }
    }
}
