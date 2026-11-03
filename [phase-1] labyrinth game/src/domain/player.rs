use crate::domain::item::Item;

pub struct Player {
    hp: u8,
    position: (usize, usize),
    bag: [Option<Item>; 2],
    total_step: usize,
    max_hp: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 5,
            position: (1, 1),
            bag: [None, None],
            total_step: 0,
            max_hp: 5,
        }
    }

    pub fn hp(&self) -> u8 {
        self.hp
    }

    pub fn max_hp(&self) -> u8 {
        self.max_hp
    }

    pub fn total_step(&self) -> usize {
        self.total_step
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn show_inventory(&self) -> &[Option<Item>; 2] {
        &self.bag
    }

    pub fn pick_up_item(&mut self, item: Item) -> bool {
        for i in 0..2 {
            if self.bag[i].is_none() {
                self.bag[i] = Some(item);
                return true;
            }
        }

        false
    }

    pub fn consume_key_if_exist(&mut self) -> bool {
        for i in 0..2 {
            if let Some(Item::Key) = self.bag[i] {
                self.bag[i].take();
                return true;
            }
        }

        false
    }

    pub fn consume_potion_or_none(&mut self) -> Option<u8> {
        for i in 0..2 {
            if let Some(Item::Potion) = self.bag[i] {
                self.bag[i].take();
                return Some(self.heal(2));
            }
        }

        None
    }

    pub fn move_position(&mut self, (r, c): (usize, usize)) {
        self.position = (r, c)
    }

    pub fn add_step(&mut self, n: usize) {
        self.total_step += n
    }

    pub fn damage(&mut self, n: u8) {
        self.hp = self.hp.saturating_sub(n)
    }

    pub fn heal(&mut self, n: u8) -> u8 {
        let current_hp = self.hp;
        self.hp = self.hp.saturating_add(n).min(self.max_hp);
        self.hp - current_hp
    }
}
