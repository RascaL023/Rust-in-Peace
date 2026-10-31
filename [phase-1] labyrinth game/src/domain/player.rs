pub struct Player {
    hp: u8,
    position: (usize, usize),
    has_key: bool,
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
            has_key: false,
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

    pub fn has_key(&self) -> bool {
        self.has_key
    }

    pub fn total_step(&self) -> usize {
        self.total_step
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn pick_up_key(&mut self) {
        self.has_key = true;
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

    pub fn heal(&mut self, n: u8) {
        self.hp = self.hp.saturating_add(n).min(self.max_hp);
    }
}
