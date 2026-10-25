use crate::domain::direction::Direction;

pub enum Command {
    Go(Direction),
    Quit,
}
