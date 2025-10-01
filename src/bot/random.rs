use crate::board::{BOARD_SIZE, Position};
use crate::bot::Bot;
use crate::game::{Command, Game};
use rand::prelude::*;

pub struct RandomBot {
    random_generator: ThreadRng,
}

impl RandomBot {
    fn propose_next_command(&mut self, game: &Game) -> Command {
        // make new stone
        let stone = game.turn.clone();

        // make new position
        let x = self.random_generator.random_range(1..=BOARD_SIZE);
        let y = self.random_generator.random_range(1..=BOARD_SIZE);
        let position = Position { x, y };

        // make new command
        let command = Command::PutStone { stone, position };

        command
    }
}

impl Bot for RandomBot {
    fn new() -> Self {
        RandomBot {
            random_generator: rand::rng(),
        }
    }

    fn next_command(&mut self, game: &Game) -> Command {
        // search randomly to put stone
        loop {
            // generate command to try
            let command = self.propose_next_command(&game);

            // copy game to try
            let mut virtual_game = game.clone();

            // if can play, return command.
            match virtual_game.play(command.clone()) {
                Ok(_) => return command,
                Err(_) => continue,
            }
        }
    }
}
