use std::fmt::{self, write};

const BOARD_SIZE: usize = 9;

fn main() {
    let board = Board::new();
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
                write!(f, "⚫")
            }
            Stone::White => {
                write!(f, "⚪️")
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
            (0..BOARD_SIZE)
                .into_iter()
                .map(|_| "──")
                .collect::<String>()
        )?;
        write!(f, "┐\n")?;

        for i in 0..BOARD_SIZE {
            // left side line
            write!(f, "│")?;
            for j in 0..BOARD_SIZE {
                if let Some(stone) = self.stones[i][j] {
                    write!(f, "{}", stone)?;
                } else {
                    write!(f, "＋")?;
                }
            }
            // right side line
            write!(f, "│\n")?;
        }

        // lower line
        write!(f, "└")?;
        write!(
            f,
            "{}",
            (0..BOARD_SIZE)
                .into_iter()
                .map(|_| "──")
                .collect::<String>()
        )?;
        write!(f, "┘\n")?;
        Ok(())
    }
}
