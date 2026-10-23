use std::io::{self, Write};

fn main() {
    // 0 = Path | 1 = Wall | 2 = Exit
    let map: [[u8; 8]; 6] = [
        [1, 1, 1, 1, 1, 1, 0, 1],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [1, 1, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 1, 1, 2, 1],
        [1, 0, 1, 1, 1, 1, 1, 1],
    ];
    // Row, Column
    let mut player_position_idx: (usize, usize) = (1, 1);
    let mut step_count: usize = 0;
    let mut input = String::new();
    let mut message: &str = "";

    loop {
        input.clear();
        simple_clear_screen();
        print_map(&map, &player_position_idx);
        println!("Step count: {step_count}");
        if map[player_position_idx.0][player_position_idx.1] == 2 {
            println!("Berhasil!");
            break;
        }

        if !message.is_empty() {
            println!("{message}");
        }

        println!("Perintah (k/j/h/l/q): ");
        print!("> ");
        io::stdout().flush().expect("Gagal flush");
        io::stdin().read_line(&mut input).expect("Tidak  bisa memproses input");

        let input = input.trim();
        if !is_valid_input(input) {
            message = "Invalid input";
            continue;
        }

        if input == "q" {
            println!("Sampai jumpa");
            break;
        }

        let (movement_row, movement_col) = parse_input_movement(input);
        let (movement_row, movement_col) = resolve_movement(&(movement_row, movement_col), &player_position_idx);
        let (movement_row, movement_col): (usize, usize) = match proceed_movement(&(movement_row, movement_col), &map) {
            None => {
                message = "Keluar map";
                continue;
            },
            Some((r, c)) => (r, c)
        };

        if !is_valid_move(&(movement_row, movement_col), &map) {
            message = "Menabrak tembok";
            continue;
        }

        player_position_idx = (movement_row, movement_col);
        step_count += 1;
        message = "";
    }
}

fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn is_valid_input(input: &str) -> bool {
    matches!(input, "j" | "k" | "l" | "h" | "q")
}

fn is_outside_map((row, col): &(i8, i8), map: &[[u8; 8]; 6]) -> bool {
    *row < 0 || *row > map.len() as i8 - 1 ||
    *col < 0 || *col > map[0].len() as i8 - 1
}

fn is_valid_move((movement_row, movement_col): &(usize, usize), map: &[[u8; 8]; 6]) -> bool {
    map[*movement_row][*movement_col] != 1
}

fn parse_input_movement(input: &str) -> (i8, i8) {
    let row_move: i8 = if input == "k" {
        -1
    } else if input == "j" {
        1
    } else {
        0
    };

    let col_move: i8 = if input == "l" {
        1
    } else if input == "h" {
        -1
    } else {
        0
    };


    (row_move, col_move)
}

fn resolve_movement(
    (movement_row, movement_col): &(i8, i8), 
    (player_pos_row, player_pos_col): &(usize, usize)
) -> (i8, i8) {
    (*player_pos_row as i8 + *movement_row, *player_pos_col as i8 + *movement_col)
}

fn proceed_movement((movement_row, movement_col): &(i8, i8), map: &[[u8; 8]; 6]) -> Option<(usize, usize)> {
    if is_outside_map(&(*movement_row, *movement_col), map) {
        return None;
    }

    Some((*movement_row as usize, *movement_col as usize))
}

fn print_map(map: &[[u8; 8]; 6], (player_row, player_column): &(usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!(" ");
            if i == *player_row && j == *player_column {
                print!("@");
            } else if map[i][j] == 0 {
                print!(".");
            } else if map[i][j] == 1 {
                print!("#");
            } else if map[i][j] == 2 {
                print!("E");
            }
        }
        println!();
    }
}
