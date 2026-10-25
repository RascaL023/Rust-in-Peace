use labyrinth::domain::command::Command;
use labyrinth::domain::direction::Direction;

pub fn parse(input: &str) -> Option<Command> {
    match input {
        "q" => Some(Command::Quit),
        "k" => Some(Command::Go(Direction::North)),
        "j" => Some(Command::Go(Direction::South)),
        "l" => Some(Command::Go(Direction::East)),
        "h" => Some(Command::Go(Direction::West)),
        _ => None,
    }
}
