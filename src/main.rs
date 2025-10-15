pub mod board;
pub mod bot;
pub mod game;

use bot::{Bot, RandomBot};
use game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new();
    let mut bot_player = RandomBot::new();
    let mut bot_player2 = RandomBot::new();

    println!("initial board");
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
        //     position: raw_command.try_into().unwrap(),
        // };
        // game.play(command).unwrap();
        // println!("{}", game.board);

        // play computer
        println!("{:?}: computer's turn.", game.turn);
        let command = bot_player2.next_command(&game);
        if game.play(command.clone()).is_err() {
            panic!("error");
        };
        println!("command: {:?}", command);
        println!("{}", game.board);

        // play computer
        println!("{:?}: computer's turn.", game.turn);
        let command = bot_player.next_command(&game);
        if game.play(command.clone()).is_err() {
            panic!("error");
        }
        println!("command: {:?}", command);
        println!("{}", game.board);
    }
}
