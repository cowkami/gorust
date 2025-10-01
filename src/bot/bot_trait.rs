use crate::game::{Command, Game};

pub trait Bot {
    fn new() -> Self;
    fn next_command(&mut self, game: &Game) -> Command;
}
