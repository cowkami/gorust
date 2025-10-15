use crate::board::{Board, Position, Stone};

#[derive(Debug, Clone)]
pub struct Game {
    pub turn: Stone,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Stone::Black,
            board: Board::new(),
        }
    }

    pub fn play(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::PutStone { stone, position } => match self.board.put(stone, position) {
                Ok(_) => {
                    self.flip_turn();
                    Ok(())
                }
                Err(err) => Err(format!("failed to execute command: {}", err)),
            },
        }
    }

    pub fn flip_turn(&mut self) {
        self.turn = self.turn.flip();
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    PutStone { stone: Stone, position: Position },
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
