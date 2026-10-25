use labyrinth::domain::game::Game;

mod terminal;

fn main() {
    let mut game = Game::default();
    terminal::run(&mut game);
}
