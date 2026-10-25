use labyrinth::domain::tile::Tile;

pub fn symbol(tile: Tile) -> char {
    match tile {
        Tile::Floor => '.',
        Tile::Wall => '#',
        Tile::Exit => 'E',
        Tile::Key => 'K',
        Tile::Door => 'D',
    }
}

// ========================== UTILS ==========================
pub fn print_info(
    map: &[[u8; 8]; 6],
    step_count: usize,
    have_key: bool,
    player_position_idx: &(usize, usize),
) {
    print_map(map, player_position_idx);
    println!("Total langkah : {step_count}");
    println!("Kunci         : {}", if have_key { "ya" } else { "tidak" });
}

pub fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_map(map: &[[u8; 8]; 6], (player_row, player_column): &(usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!(" ");
            if i == *player_row && j == *player_column {
                print!("@");
                continue;
            }

            match Tile::from_code(map[i][j]) {
                Some(t) => print!("{}", symbol(t)),
                None => print!("?"),
            }
        }
        println!();
    }
}
