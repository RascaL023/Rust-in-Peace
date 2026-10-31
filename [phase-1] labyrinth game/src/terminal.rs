mod input;
mod render;

use std::io::{self, Write};

use labyrinth::domain::{command::Command, game::Game, outcome::Outcome};

pub fn run(game: &mut Game) {
    let mut input = String::new();
    let mut message: &str = "";
    let mut hp_change: i8 = -1;

    loop {
        input.clear();
        draw_frame(game);

        if hp_change != -1 {
            println!("{message}{hp_change}.");
        } else if !message.is_empty() {
            println!("{message}");
        }

        message = "";
        hp_change = -1;
        println!("Perintah (k/j/h/l/q): ");
        print!("> ");
        io::stdout().flush().expect("Gagal flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Tidak  bisa memproses input");

        let (movement_row, movement_col) = match input::parse(input.trim()) {
            Some(Command::Quit) => {
                println!("Sampai jumpa");
                break;
            }
            Some(Command::Go(d)) => {
                let (r, c) = d.vector_from();
                let (rp, cp) = game.player_position();
                (rp as i8 + r, cp as i8 + c)
            }
            None => {
                message = "Invalid input";
                continue;
            }
        };

        match game.proceed_movement(movement_row, movement_col) {
            Outcome::Moved => {}
            Outcome::HitWall => {
                message = "Menabrak tembok";
            }
            Outcome::OutOfMap => {
                message = "Keluar map";
            }

            Outcome::TookKey => {
                message = "Kunci berhasil diambil";
            }
            Outcome::OpenedDoor => {
                message = "Pintu terbuka";
            }
            Outcome::DoorLocked => {
                message = "Pintu terkunci";
            }

            Outcome::Hurt(n) => {
                message = "Jebakan! HP -";
                hp_change = n as i8;
            }
            Outcome::Healed(n) => {
                message = "Ramuan! HP +";
                hp_change = n as i8;
            }

            Outcome::Won => {
                draw_frame(game);

                println!("Berhasil");
                break;
            }
            Outcome::Died => {
                draw_frame(game);

                println!("Game Over! Kamu gagal :v");
                break;
            }
        }
    }
}

fn draw_frame(game: &Game) {
    render::simple_clear_screen();
    render::draw_map(game);
    render::display_info(game);
}
