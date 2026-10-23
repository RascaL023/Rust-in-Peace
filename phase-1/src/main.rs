use std::{io::{self, Write}, process::exit};

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
    let mut player_position_idx: (usize, usize) = (5, 1);
    let mut step_count: u8 = 0;
    let mut input = String::new();

    loop {
        simple_clear_screen();
        print_map(&map, &player_position_idx);
        println!("Step count: {step_count}");
        input.clear();

        print!("Input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let (movement_row, movement_col) = match resolve_movement_with_execution(input.trim(), &player_position_idx, &map) {
            Some((r, c)) => {
                if r + c == 0 {
                    println!("Nabrak!");
                    continue;
                }

                if map[r][c] == 2 {
                    println!("Congrats");
                    player_position_idx = (r, c);
                    step_count += 1;
                    break;
                }
                (r, c)
            },
            None => {
                println!("Keluar dari map");
                continue;
            },
        };

        player_position_idx = (movement_row, movement_col);
        step_count += 1;
    }

    simple_clear_screen();
    print_map(&map, &player_position_idx);
    println!("Step count: {step_count}");
}

fn simple_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn resolve_movement_with_execution(input: &str, (player_row, player_col): &(usize, usize), map: &[[u8; 8]; 6]) -> Option<(usize, usize)> {
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

    let col_move: i8 = if input == "l" {
        1
    } else if input == "h" {
        -1
    } else {
        0
    };

    let index_row: i8 = *player_row as i8 + row_move;
    let index_col: i8 = *player_col as i8 + col_move;
    if index_row < 0 || index_row > map.len() as i8 - 1 ||
    index_col < 0 || index_col > map[0].len() as i8 - 1 {
        return None;
    } else if map[index_row as usize][index_col as usize] == 1 {
        return Some((0 , 0));
    } 

    Some((index_row as usize, index_col as usize))
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
