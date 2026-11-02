use labyrinth::domain::command::Command;
use labyrinth::domain::direction::Direction;

pub fn parse(input: &str) -> Option<Command> {
    match input {
        "i" => Some(Command::ShowInventory),
        "p" => Some(Command::ConsumePotion),
        "k" => Some(Command::Go(Direction::North)),
        "j" => Some(Command::Go(Direction::South)),
        "l" => Some(Command::Go(Direction::East)),
        "h" => Some(Command::Go(Direction::West)),
        "q" => Some(Command::Quit),
        _ => None,
    }
}
