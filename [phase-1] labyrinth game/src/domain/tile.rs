pub enum Tile {
    Floor,
    Wall,
    Exit,
    Key,
    Door,
}

impl Tile {
    pub fn from_code(code: u8) -> Option<Tile> {
        match code {
            0 => Some(Tile::Floor),
            1 => Some(Tile::Wall),
            2 => Some(Tile::Exit),
            3 => Some(Tile::Key),
            4 => Some(Tile::Door),
            _ => None,
        }
    }
}
