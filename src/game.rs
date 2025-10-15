use crate::board::{Board, Point, Stone};

#[derive(Debug, Clone)]
pub struct Game {
    pub turn: Stone,
    pub board: Board,
    pass_count: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Stone::Black,
            board: Board::new(),
            pass_count: 0,
        }
    }

    pub fn play(&mut self, command: Command) -> Result<(), String> {
        // double pass means game set
        if self.pass_count == 2 {
            return Ok(());
        }
        match command {
            Command::Move { stone, point } => match self.board.put(stone, point) {
                Ok(_) => {
                    self.flip_turn();
                    // game continues as long as someone puts stone
                    self.pass_count = 0;
                    Ok(())
                }
                Err(err) => Err(format!("failed to execute command: {}", err)),
            },
            Command::Pass => {
                self.flip_turn();
                self.pass_count += 1;
                Ok(())
            }
        }
    }

    pub fn flip_turn(&mut self) {
        self.turn = self.turn.flip();
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Move { stone: Stone, point: Point },
    Pass,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_flip_turn() {
        let mut game = Game::new();
        let previous_stone = game.turn.clone();
        game.flip_turn();
        let new_stone = game.turn.clone();

        assert_eq!(previous_stone, Stone::Black);
        assert_eq!(new_stone, Stone::White);
    }
}
