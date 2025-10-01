use crate::board::{Board, Position, Stone};

#[derive(Debug, Copy, Clone)]
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
            Command::PutStone { stone, position } => {
                self.board
                    .put(stone, &position)
                    .expect("failed to put stone");
                self.flip_turn();
                Ok(())
            }
        }
    }

    pub fn flip_turn(&mut self) {
        self.turn = match self.turn {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

pub enum Command {
    PutStone { stone: Stone, position: Position },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_flip_turn() {
        let mut game = Game::new();

        game.flip_turn();
    }
}
