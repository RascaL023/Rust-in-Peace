mod input;
mod render;

use std::io::{self, Write};

use labyrinth::domain::{command::Command, game::Game, outcome::Outcome};

pub fn run(game: &mut Game) {
    let mut input = String::new();
    let mut feedback: Option<render::Feedback> = None;

    loop {
        input.clear();
        render::draw_frame(game);

        if let Some(f) = &feedback {
            render::print_feedback(f);
        }
        if matches!(
            feedback,
            Some(render::Feedback::Game(Outcome::Won | Outcome::Died(_)))
        ) {
            break;
        }

        println!("Perintah (i/p/k/j/h/l/q): ");
        print!("> ");
        io::stdout().flush().expect("Gagal flush");
        io::stdin()
            .read_line(&mut input)
            .expect("Tidak  bisa memproses input");

        let (movement_row, movement_col, command) = match input::parse(input.trim()) {
            Some(Command::ShowInventory) => {
                let inventory = game.show_inventory();
                let displayed_inventory: [String; 2] = [
                    match inventory[0] {
                        None => String::from("Kosong"),
                        Some(labyrinth::domain::item::Item::Key) => String::from("Kunci"),
                        Some(labyrinth::domain::item::Item::Potion) => String::from("Ramuan"),
                    },
                    match inventory[1] {
                        None => String::from("Kosong"),
                        Some(labyrinth::domain::item::Item::Key) => String::from("Kunci"),
                        Some(labyrinth::domain::item::Item::Potion) => String::from("Ramuan"),
                    }
                ];
                feedback = Some(render::Feedback::Game(Outcome::ShowInventory(displayed_inventory)));
                continue;
            }
            Some(Command::ConsumePotion) => {
                (0, 0, Command::ConsumePotion)
            }
            Some(Command::Go(d)) => {
                let (r, c) = d.vector_from();
                let (rp, cp) = game.player_position();
                (rp as i8 + r, cp as i8 + c, Command::Go(d))
            }
            Some(Command::Quit) => {
                println!("Sampai jumpa");
                break;
            }
            None => {
                feedback = Some(render::Feedback::Invalid);
                continue;
            }
        };

        feedback = Some(render::Feedback::Game(
            game.proceed_movement(movement_row, movement_col, command),
        ));
    }
}
