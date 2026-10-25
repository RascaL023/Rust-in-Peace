pub enum Tile {
    Floor,
    Wall,
    Exit,
    Key,
    Door,
    Trap(u8),
    Potion(u8),
}

impl Tile {
    pub fn from_code(code: u8) -> Option<Tile> {
        match code {
            0 => Some(Tile::Floor),
            1 => Some(Tile::Wall),
            2 => Some(Tile::Exit),
            3 => Some(Tile::Key),
            4 => Some(Tile::Door),
            5 => Some(Tile::Trap(2)),
            6 => Some(Tile::Potion(3)),
            _ => None,
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Tile::Floor => 0,
            Tile::Wall => 1,
            Tile::Exit => 2,
            Tile::Key => 3,
            Tile::Door => 4,
            Tile::Trap(_) => 5,
            Tile::Potion(_) => 6,
        }
    }
}
