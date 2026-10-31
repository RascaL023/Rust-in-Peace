use labyrinth::domain::{game::Game, tile::Tile};

pub fn symbol(tile: Tile) -> char {
    match tile {
        Tile::Floor => '.',
        Tile::Wall => '#',
        Tile::Exit => 'E',
        Tile::Key => 'K',
        Tile::Door => 'D',
        Tile::Trap(_) => '^',
        Tile::Potion(_) => '+',
    }
}

pub fn display_info(game: &Game) {
    println!(
        "HP: {}/{} | Langkah: {} | Kunci: {}",
        game.player_hp(),
        game.player_max_hp(),
        game.steps(),
        if game.has_key() { "Ya" } else { "Tidak" },
    );
}

pub fn draw_map(game: &Game) {
    let (player_row, player_col) = game.player_position();
    let (map_row, map_col) = game.map_len();
    for i in 0..map_row {
        for j in 0..map_col {
            print!(" ");
            if i == player_row && j == player_col {
                print!("@");
                continue;
            }

            match game.tile_at((i, j)) {
                Some(t) => print!("{}", symbol(t)),
                None => print!("?"),
            }
        }
        println!();
    }
}

// ========================== UTILS ==========================

pub fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
