mod terminal;

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
    let player_position_idx: (usize, usize) = (1, 1);
    let step_count: usize = 0;
    let have_key = false;

    terminal::run(&mut map, player_position_idx, step_count, have_key);
}
