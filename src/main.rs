pub mod board;
pub mod game;

use game::{Command, Game};
use std::io;
fn main() -> Result<(), String> {
    let mut game = Game::new();
    println!("initial board");
    println!("{}", game.board);

    loop {
        let mut raw_command = String::new();
        io::stdin()
            .read_line(&mut raw_command)
            .expect("failed to read line");
        let command = Command::PutStone {
            stone: game.turn,
            position: raw_command.try_into().unwrap(),
        };
        game.play(command).unwrap();
        println!("{}", game.board);
    }
}
