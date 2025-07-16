use regex::Regex;
use std::fmt;
use std::io;

const BOARD_SIZE: usize = 9;
const BLACK: &str = "○";
const WHITE: &str = "●";
const NUMBERS: [char; 19] = [
    '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', //
    '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱', '⑲',
];

fn main() -> Result<(), String> {
    let mut board = Board::new();
    println!("initial board");
    println!("{}", board);

    let mut command = String::new();
    println!("{}", command);

    io::stdin()
        .read_line(&mut command)
        .expect("failed to read line");

    println!("{}", command);

    Ok(())
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

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<String> for Point {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"([0-9]+) ([0-9]+)").expect("failed to parse the String to Point");
        for (_, [x, y]) in re.captures_iter(&value).map(|c| c.extract()) {
            return Ok(Self {
                x: x.parse::<usize>().expect("failed to parse number"),
                y: y.parse::<usize>().expect("failed to parse number"),
            });
        }
        Err("failed to parse, Point pattern not found in the String")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_try_from() {
        let given = "0 0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { x: 0, y: 0 });

        let given = "1 0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { x: 1, y: 0 });

        let given = "10 0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { x: 10, y: 0 });

        let given = "10 10".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { x: 10, y: 10 });

        // failure case
        let given = "abc".to_string();
        let result = Point::try_from(given);
        assert!(result.is_err());
    }
}
