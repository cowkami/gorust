use crate::bot::Bot;
use crate::game::{Command, Game};
use rand::prelude::*;

pub struct RandomBot {
    random_generator: ThreadRng,
}

impl Bot for RandomBot {
    fn new() -> Self {
        RandomBot {
            random_generator: rand::rng(),
        }
    }

    fn next_command(&mut self, game: &Game) -> Command {
        // to prevent mutation of the game, clone the game
        let mut virtual_game = game.clone();
        // todo: refactor not to use Board directly
        let available_points = virtual_game.board.find_available_points(game.turn);
        let random_point = available_points.choose(&mut self.random_generator);
        // make new command.
        // if there is some available point, put stone.
        // if no, pass the turn
        match random_point {
            Some(&point) => Command::PutStone {
                stone: game.turn,
                point,
            },
            None => Command::Pass,
        }
    }
}
