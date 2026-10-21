use std::{io::{self, Write}, process::exit};

fn main() {
    // 0 = Path | 1 = Wall | 2 = Exit
    let map: [[u8; 8]; 6] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1, 0, 0, 1],
        [1, 1, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 1, 1, 2, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];
    // Row, Column
    let mut player_position_idx: (u8, u8) = (1, 1);
    let mut step_count = 0;
    let mut input = String::new();

    loop {
        simple_clear_screen();
        print_map(&map, &player_position_idx);
        input.clear();

        print!("Input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let movement: (i8, i8) = resolve_movement(input.trim());
        if movement == (0, 0) {
            continue;
        }

        if validate_movement(&player_position_idx, &movement, &map) {
            player_position_idx = ((movement.0 + player_position_idx.0 as i8).try_into().unwrap(), (movement.1 + player_position_idx.1 as i8).try_into().unwrap());
        } else {
            println!("Invalid");
            continue;
        }

        step_count += 1;

        if step_count == 8 {
            break;
        }
    }

    simple_clear_screen();
    print_map(&map, &player_position_idx);
    println!("Step count: {step_count}");
}

fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn validate_movement((player_row, player_col): &(u8, u8), (next_move_row, next_move_col): &(i8, i8), map: &[[u8; 8]; 6]) -> bool {
    let index_row: i8 = *player_row as i8 + next_move_row;
    let index_col: i8 = *player_col as i8 + next_move_col;
    if index_row < 0 || index_col < 0 {
        return false
    }

    let next_move = map[index_row as usize][index_col as usize];

    !(next_move == 1)
}

fn resolve_movement(input: &str) -> (i8, i8) {
    if input == "q" {
        println!("Sampai jumpa");
        exit(0);
    }

    let row_move: i8 = if input == "k" {
        -1
    } else if input == "j" {
        1
    } else {
        0
    };

    let column_move: i8 = if input == "l" {
        1
    } else if input == "h" {
        -1
    } else {
        0
    };

    (row_move, column_move)
}

fn print_map(map: &[[u8; 8]; 6], (player_row, player_column): &(u8, u8)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!(" ");
            if i as u8 == *player_row && j as u8 == *player_column {
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
