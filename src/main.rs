use std::fmt;

const BOARD_SIZE: usize = 9;
const BLACK: &str = "○";
const WHITE: &str = "●";

fn main() {
    let mut board = Board::new();
    board.stones[4][4] = Some(Stone::Black);
    board.stones[4][5] = Some(Stone::White);
    board.stones[4][3] = Some(Stone::White);
    board.stones[3][3] = Some(Stone::White);
    board.stones[0][0] = Some(Stone::White);
    println!("{}", board);
}

#[derive(Copy, Clone)]
enum Stone {
    Black,
    White,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stone::Black => {
                write!(f, "{}", BLACK)
            }
            Stone::White => {
                write!(f, "{}", WHITE)
            }
        }
    }
}

struct Board {
    stones: [[Option<Stone>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board {
            stones: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // upper line
        write!(f, "┌")?;
        write!(
            f,
            "{}",
            (0..BOARD_SIZE * 2 + 5)
                .into_iter()
                .map(|_| "─")
                .collect::<String>()
        )?;
        write!(f, "┐\n")?;

        // upper coordination
        write!(f, "│  ")?;
        for i in 0..BOARD_SIZE {
            write!(f, " {}", i + 1)?;
        }
        write!(f, "   │\n")?;

        for i in 0..BOARD_SIZE {
            // left side line
            write!(f, "│ {} ", i + 1)?;
            for j in 0..BOARD_SIZE {
                if let Some(stone) = self.stones[i][j] {
                    write!(f, "{} ", stone)?;
                } else {
                    if i == 0 && j == 0 {
                        write!(f, "{}", "┌─")?;
                    } else if i == 0 && j == BOARD_SIZE - 1 {
                        write!(f, "{}", "┐")?;
                    } else if i == BOARD_SIZE - 1 && j == 0 {
                        write!(f, "└─")?;
                    } else if i == BOARD_SIZE - 1 && j == BOARD_SIZE - 1 {
                        write!(f, "┘")?;
                    } else if i == 0 && j != 0 {
                        write!(f, "{}", "┬─")?;
                    } else if i == BOARD_SIZE - 1 && j != 0 {
                        write!(f, "{}", "┴─")?;
                    } else if i != 0 && j == 0 {
                        write!(f, "{}", "├─")?;
                    } else if i != 0 && j == BOARD_SIZE - 1 {
                        write!(f, "{}", "┤")?;
                    } else {
                        write!(f, "{}", "┼─")?;
                    }
                }
            }
            // right side line
            write!(f, " {} │\n", i + 1)?;
        }

        // lower coordination
        write!(f, "│  ")?;
        for i in 0..BOARD_SIZE {
            write!(f, " {}", i + 1)?;
        }
        write!(f, "   │\n")?;

        // lower line
        write!(f, "└")?;
        write!(
            f,
            "{}",
            (0..BOARD_SIZE * 2 + 5)
                .into_iter()
                .map(|_| "─")
                .collect::<String>()
        )?;
        write!(f, "┘\n")?;
        Ok(())
    }
}
