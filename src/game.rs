use crate::board::{BOARD_SIZE, Board, BoardCell, Point, Stone};

#[derive(Debug, Clone)]
pub struct Game {
    pub turn: Stone,
    pub board: Board,
    pub status: GameStatus,
    pub komi: f32,
    pass_count: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Stone::Black,
            board: Board::new(),
            status: GameStatus::Continue,
            komi: 3.75,
            pass_count: 0,
        }
    }

    pub fn play(&mut self, command: Command) -> Result<(), String> {
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
                // double pass means game set
                if self.pass_count == 2 {
                    self.status = GameStatus::End;
                }
                Ok(())
            }
        }
    }

    pub fn flip_turn(&mut self) {
        self.turn = self.turn.flip();
    }

    pub fn game_end(&mut self) -> GameResult {
        let mut black = 0.0;
        let mut white = 0.0;
        for row in 1..=BOARD_SIZE as i8 {
            for col in 1..=BOARD_SIZE as i8 {
                match self.board.get(Point { row, col }) {
                    BoardCell::Space(Some(Stone::Black)) => {
                        black += 1.0;
                    }
                    BoardCell::Space(Some(Stone::White)) => {
                        white += 1.0;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        white += self.komi;
        println!("black stones: {}", black);
        println!("white stones: {}", white);
        println!("black prisoners: {:?}", self.board.black_prisoners);
        println!("white prisoners: {:?}", self.board.white_prisoners);
        GameResult {
            black,
            white,
            winner: if black > white {
                Winner::Black
            } else if black < white {
                Winner::White
            } else {
                Winner::Draw
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move { stone: Stone, point: Point },
    Pass,
}

#[derive(Debug, Clone)]
pub enum GameStatus {
    Continue,
    End,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    black: f32,
    white: f32,
    winner: Winner,
}

#[derive(Debug, Clone)]
pub enum Winner {
    Black,
    White,
    Draw,
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
