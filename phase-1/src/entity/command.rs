use crate::entity::direction::Direction;

pub enum Command {
    Go(Direction),
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Option<Command> {
        match input {
            "q" => Some(Self::Quit),
            "k" => Some(Self::Go(Direction::North)),
            "j" => Some(Self::Go(Direction::South)),
            "l" => Some(Self::Go(Direction::East)),
            "h" => Some(Self::Go(Direction::West)),
            _ => None,
        }
    }
}
