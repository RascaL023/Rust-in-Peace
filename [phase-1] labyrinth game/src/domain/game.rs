use crate::domain::{outcome::Outcome, player::Player, tile::Tile};

pub struct Game {
    pub map: [[u8; 8]; 6],
    pub player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    fn new() -> Game {
        Game {
            map: [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1, 3, 6, 1],
                [1, 1, 1, 5, 1, 5, 1, 1],
                [1, 0, 5, 0, 0, 0, 4, 2],
                [1, 0, 1, 1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
            player: Player::default(),
        }
    }

    pub fn player_position(&self) -> (usize, usize) {
        self.player.position
    }

    pub fn hp(&self) -> u8 {
        self.player.hp()
    }

    pub fn steps(&self) -> usize {
        self.player.total_step
    }

    pub fn has_key(&self) -> bool {
        self.player.has_key()
    }

    pub fn proceed_movement(&mut self, row: i8, col: i8) -> Outcome {
        let (r, c): (usize, usize) = (row as usize, col as usize);
        match self.tile_at(row, col) {
            Some(Tile::Trap(n)) => {
                self.moved_to(r, c);
                self.player.damage(n);
                if self.hp() == 0 {
                    return Outcome::Died;
                };

                Outcome::Hurt(n)
            }
            Some(Tile::Potion(n)) => {
                let current_hp = self.hp();

                self.moved_to(r, c);
                self.player.heal(n);
                self.update_map(r, c, Tile::Floor);

                Outcome::Healed(if current_hp + n > self.player.max_hp() {
                    self.player.max_hp() - current_hp
                } else {
                    n
                })
            }
            Some(Tile::Key) => {
                self.player.pick_up_key();
                self.moved_to(r, c);
                self.update_map(r, c, Tile::Floor);

                Outcome::TookKey
            }
            Some(Tile::Door) => {
                if !self.has_key() {
                    return Outcome::DoorLocked;
                }

                // self.update_map(r, c, Tile::Floor);
                self.moved_to(r, c);

                Outcome::OpenedDoor
            }
            Some(Tile::Wall) => Outcome::HitWall,
            Some(Tile::Floor) => {
                self.moved_to(r, c);

                Outcome::Moved
            }
            Some(Tile::Exit) => {
                self.moved_to(r, c);

                Outcome::Won
            }
            None => Outcome::OutOfMap,
        }
    }


    fn tile_at(&self, row: i8, col: i8) -> Option<Tile> {
        if self.is_outside_map((row, col)) {
            return None;
        }

        match Tile::from_code(self.map[row as usize][col as usize]) {
            Some(Tile::Trap(d)) => Some(Tile::Trap(d)),
            Some(Tile::Potion(h)) => Some(Tile::Potion(h)),
            Some(t) => Some(t),
            None => None,
        }
    }

    fn moved_to(&mut self, row: usize, col: usize) {
        self.player.position = (row, col);
        self.player.total_step += 1;
    }

    fn update_map(&mut self, row: usize, col: usize, tile: Tile) {
        self.map[row][col] = Tile::to_code(&tile);
    }

    fn is_outside_map(&self, (r, c): (i8, i8)) -> bool {
        r < 0 || r > self.map.len() as i8 - 1 || c < 0 || c > self.map[0].len() as i8 - 1
    }
}
