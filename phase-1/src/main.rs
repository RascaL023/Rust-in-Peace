use std::io::{self, Write};

use crate::entity::command::Command;
use crate::entity::direction::Direction;
use crate::entity::tile::Tile;

mod entity;

fn main() {
    // 0 = Path | 1 = Wall | 2 = Exit
    // 3 = Key | 4 = Door
    let mut map: [[u8; 8]; 6] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1, 3, 0, 1],
        [1, 1, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 4, 2],
        [1, 0, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    // Row, Column
    let mut player_position_idx: (usize, usize) = (1, 1);
    let mut step_count: usize = 0;
    let mut input = String::new();
    let mut message: &str = "";

    let mut have_key = false;

    loop {
        input.clear();
        simple_clear_screen();
        print_info(&map, step_count, have_key, &player_position_idx);

        if !message.is_empty() {
            println!("{message}");
        }

        message = "";
        println!("Perintah (k/j/h/l/q): ");
        print!("> ");
        io::stdout().flush().expect("Gagal flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Tidak  bisa memproses input");

        let input = input.trim();
        let (movement_row, movement_col) = match Command::parse(input) {
            Some(Command::Quit) => {
                println!("Sampai jumpa");
                break;
            }
            Some(Command::Go(Direction::North)) => (-1, 0),
            Some(Command::Go(Direction::South)) => (1, 0),
            Some(Command::Go(Direction::East)) => (0, 1),
            Some(Command::Go(Direction::West)) => (0, -1),
            None => {
                message = "Invalid input";
                continue;
            }
        };

        let (movement_row, movement_col) =
            resolve_movement(&(movement_row, movement_col), &player_position_idx);
        let (movement_row, movement_col): (usize, usize) =
            match proceed_movement(&(movement_row, movement_col), &map) {
                None => {
                    message = "Keluar map";
                    continue;
                }
                Some((r, c)) => (r, c),
            };

        match Tile::tile_from(map[movement_row][movement_col]) {
            Some(Tile::Exit) => {
                simple_clear_screen();
                update_movement(&mut player_position_idx, (movement_row, movement_col), &mut step_count);
                print_info(&map, step_count, have_key, &player_position_idx);

                println!("Berhasil!");
                break;
            }
            Some(Tile::Floor) => {}
            Some(Tile::Wall) => {
                message = "Menabrak tembok";
                continue;
            }
            Some(Tile::Key) => {
                key_received(&mut map, (movement_row, movement_col));
                have_key = true;
                message = "Kunci berhasil diambil";
            }
            Some(Tile::Door) => {
                if have_key {
                    message = "Pintu terbuka";
                } else {
                    message = "Pintu terkunci";
                    continue;
                }
            }
            None => {
                message = "Tile tidak dikenali";
                continue;
            }
        };

        update_movement(&mut player_position_idx, (movement_row, movement_col), &mut step_count);
    }
}

// Count current position + move position, return index position
fn resolve_movement(
    (movement_row, movement_col): &(i8, i8),
    (player_pos_row, player_pos_col): &(usize, usize),
) -> (i8, i8) {
    (
        *player_pos_row as i8 + *movement_row,
        *player_pos_col as i8 + *movement_col,
    )
}

fn proceed_movement(
    (movement_row, movement_col): &(i8, i8),
    map: &[[u8; 8]; 6],
) -> Option<(usize, usize)> {
    if is_outside_map(&(*movement_row, *movement_col), map) {
        return None;
    }

    Some((*movement_row as usize, *movement_col as usize))
}

fn key_received(map: &mut [[u8; 8]; 6], (row, col): (usize, usize)) {
    map[row][col] = 0;
}

fn update_movement(player_position_idx: &mut (usize, usize), new_move: (usize, usize), step_count: &mut usize) {
    *player_position_idx = new_move;
    *step_count += 1;
}

// ========================== UTILS ==========================
fn print_info(map: &[[u8; 8]; 6], step_count: usize, have_key: bool, player_position_idx: &(usize, usize)) {
    print_map(map, player_position_idx);
    println!("Total langkah : {step_count}");
    println!("Kunci         : {}", if have_key { "ya" } else { "tidak" });
}

fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn is_outside_map((row, col): &(i8, i8), map: &[[u8; 8]; 6]) -> bool {
    *row < 0 || *row > map.len() as i8 - 1 || *col < 0 || *col > map[0].len() as i8 - 1
}

fn print_map(map: &[[u8; 8]; 6], (player_row, player_column): &(usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!(" ");
            if i == *player_row && j == *player_column {
                print!("@");
                continue;
            }

            match Tile::tile_from(map[i][j]) {
                Some(t) => print!("{}", t.symbol()),
                None => print!("?"),
            }
        }
        println!();
    }
}
