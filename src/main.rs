pub mod board;
pub mod bot;
pub mod frac;
pub mod game;

use bot::{Bot, RandomBot};
use game::Game;

use crate::game::GameStatus;

fn main() -> Result<(), String> {
    let mut game = Game::new();
    let mut bot_player = RandomBot::new();
    let mut bot_player2 = RandomBot::new();

    println!("game start.");
    println!("{}", game.board);

    loop {
        // play human
        // println!("your turn.");
        // let mut raw_command = String::new();
        // io::stdin()
        //     .read_line(&mut raw_command)
        //     .expect("failed to read line");
        // let command = Command::PutStone {
        //     stone: game.turn,
        //     point: raw_command.try_into().unwrap(),
        // };
        // game.play(command).unwrap();
        // println!("{}", game.board);

        // play computer
        println!("{:?}: computer's turn.", game.turn);
        let command = bot_player2.next_command(&game);
        if game.play(command).is_err() {
            panic!("error");
        };
        println!("command: {:?}", command);
        println!("{}", game.board);
        if matches!(game.status, GameStatus::End) {
            break;
        }

        // play computer
        println!("{:?}: computer's turn.", game.turn);
        let command = bot_player.next_command(&game);
        if game.play(command).is_err() {
            panic!("error");
        }
        println!("command: {:?}", command);
        println!("{}", game.board);
        if matches!(game.status, GameStatus::End) {
            break;
        }
    }
    println!("{:?}", game.game_end());
    println!("game end.");

    Ok(())
}
