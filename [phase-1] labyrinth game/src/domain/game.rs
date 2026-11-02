use crate::domain::{command::Command, item::Item, outcome::Outcome, player::Player, tile::Tile};

pub struct Game {
    map: [[u8; 8]; 6],
    player: Player,
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
                [1, 0, 6, 6, 1, 3, 6, 1],
                [1, 1, 1, 6, 1, 5, 1, 1],
                [1, 0, 5, 0, 0, 0, 4, 2],
                [1, 6, 1, 1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
            player: Player::default(),
        }
    }

    pub fn map_len(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn player_position(&self) -> (usize, usize) {
        self.player.position()
    }

    pub fn player_max_hp(&self) -> u8 {
        self.player.max_hp()
    }

    pub fn player_hp(&self) -> u8 {
        self.player.hp()
    }

    pub fn steps(&self) -> usize {
        self.player.total_step()
    }

    pub fn has_key(&self) -> bool {
        self.player.has_key()
    }

    pub fn show_inventory(&self) -> &[Option<Item>; 2] {
        self.player.show_inventory()
    }

    pub fn tile_at(&self, (r, c): (usize, usize)) -> Option<Tile> {
        if self.is_outside_map((r as i8, c as i8)) {
            return None;
        }

        Tile::from_code(self.map[r][c])
    }

    pub fn player_consume_potion(&mut self) -> Outcome {
        if let Some(n) = self.player.consume_potion_or_none() {
            return Outcome::Healed(n);
        }

        Outcome::DontHavePotion
    }

    pub fn proceed_movement(&mut self, row: i8, col: i8, command: Command) -> Outcome {
        if let Command::ConsumePotion = command {
            return self.player_consume_potion();
        } else if self.is_outside_map((row, col)) {
            return Outcome::OutOfMap;
        }

        let (r, c): (usize, usize) = (row as usize, col as usize);
        match Tile::from_code(self.map[r][c]) {
            Some(Tile::Trap(n)) => {
                let current_hp = self.player_hp();
                self.moved_to(r, c);
                self.player.damage(n);
                let current_hp = current_hp - self.player_hp();
                if self.player_hp() == 0 {
                    return Outcome::Died(current_hp);
                };

                Outcome::Hurt(current_hp)
            }
            Some(Tile::Potion(_)) => {
                // let current_hp = self.player_hp();

                self.moved_to(r, c);
                if !self.player.pick_up_item(Item::Potion) {
                    return Outcome::InventoryFull;
                }
                self.update_map(r, c, Tile::Floor);

                // Outcome::Healed(self.player_hp() - current_hp)
                Outcome::TookPotion
            }
            Some(Tile::Key) => {
                self.moved_to(r, c);
                if !self.player.pick_up_item(Item::Key) {
                    return Outcome::InventoryFull;
                }
                self.update_map(r, c, Tile::Floor);

                Outcome::TookKey
            }
            Some(Tile::Door) => {
                if !self.player.consume_key_if_exist() {
                    return Outcome::DoorLocked;
                }

                self.update_map(r, c, Tile::Floor);
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

    fn moved_to(&mut self, row: usize, col: usize) {
        self.player.move_position((row, col));
        self.player.add_step(1);
    }

    fn update_map(&mut self, row: usize, col: usize, tile: Tile) {
        self.map[row][col] = Tile::to_code(&tile);
    }

    fn is_outside_map(&self, (r, c): (i8, i8)) -> bool {
        r < 0 || r > self.map.len() as i8 - 1 || c < 0 || c > self.map[0].len() as i8 - 1
    }
}
