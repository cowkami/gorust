use std::fmt;

const BOARD_SIZE: usize = 9;
const BLACK: &str = "○";
const WHITE: &str = "●";
const NUMBERS: [char; 19] = [
    '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', //
    '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱', '⑲',
];

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
            write!(f, " {}", NUMBERS[i])?;
        }
        write!(f, "   │\n")?;

        for i in 0..BOARD_SIZE {
            // left side line
            write!(f, "│ {} ", NUMBERS[i])?;
            for j in 0..BOARD_SIZE {
                if let Some(stone) = self.stones[i][j] {
                    write!(f, "{} ", stone)?;
                } else {
                    if i == 0 && j == 0 {
                        write!(f, "{}", "┌─")?;
                    // draw hoshi
                    } else if BOARD_SIZE == 9
                        && (
                            // draw hoshi at sumi
                            (i == 2 && j == 2)
                            || (i == 2 && j == BOARD_SIZE - 3)
                            || (i == BOARD_SIZE - 3 && j == 2)
                            || (i == BOARD_SIZE - 3 && j == BOARD_SIZE - 3)
                            // draw tengen
                            || (i == BOARD_SIZE / 2 && j == BOARD_SIZE / 2)
                        )
                    {
                        write!(f, "•─")?;
                    } else if (BOARD_SIZE == 13 || BOARD_SIZE == 19)
                        && (
                            // draw hoshi at sumi
                            (i == 3 && j == 3)
                            || (i == 3 && j == BOARD_SIZE - 4)
                            || (i == BOARD_SIZE - 4 && j == 3)
                            || (i == BOARD_SIZE - 4 && j == BOARD_SIZE - 4)
                            // draw tengen
                            || (i == BOARD_SIZE / 2 && j == BOARD_SIZE / 2)
                            // drow hoshi at hen
                            || (i == 3 && j == BOARD_SIZE / 2)
                            || (i == BOARD_SIZE / 2 && j == 3)
                            || (i == BOARD_SIZE - 4 && j == BOARD_SIZE / 2)
                            || (i == BOARD_SIZE / 2 && j == BOARD_SIZE - 4)
                        )
                    {
                        write!(f, "•─")?;
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
            write!(f, " {} │\n", NUMBERS[i])?;
        }

        // lower coordination
        write!(f, "│  ")?;
        for i in 0..BOARD_SIZE {
            write!(f, " {}", NUMBERS[i])?;
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
