use regex::Regex;
use std::{cmp, collections::HashSet, fmt};

pub const BOARD_SIZE: usize = 9;
const BLACK: &str = "○";
const WHITE: &str = "●";
const NUMBERS: [char; 19] = [
    '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', //
    '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱', '⑲',
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn flip(self) -> Stone {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

pub enum BoardCell {
    Wall,
    Space(Option<Stone>),
}

#[derive(Debug, Clone)]
pub struct Board {
    space: [[Option<Stone>; BOARD_SIZE]; BOARD_SIZE],
    pub black_prisoners: usize,
    pub white_prisoners: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            space: [[None; BOARD_SIZE]; BOARD_SIZE],
            black_prisoners: 0,
            white_prisoners: 0,
        }
    }

    pub fn new_with_prisoners(black_prisoners: usize, white_prisoners: usize) -> Board {
        Board {
            space: [[None; BOARD_SIZE]; BOARD_SIZE],
            black_prisoners,
            white_prisoners,
        }
    }

    fn get(&self, position: Position) -> BoardCell {
        // check position range
        if position.row <= 0
            || BOARD_SIZE < position.row as usize
            || position.col <= 0
            || BOARD_SIZE < position.col as usize
        {
            BoardCell::Wall
        } else {
            BoardCell::Space(self.space[position.row as usize - 1][position.col as usize - 1])
        }
    }

    pub fn put(&mut self, stone: Stone, position: Position) -> Result<(), String> {
        // validate for go rule
        match self.can_put(stone, position.clone()) {
            Ok(_) => {
                // if the stone kills the opponent's stones,
                // if self.can_kill(stone, position) {
                //     // take these stones from the board,
                //     if self.kill(stone, position).is_err() {
                //         return Err("cannot kill stones".to_string());
                //     }
                // }
                // put the stone on the position
                self.space[position.row as usize - 1][position.col as usize - 1] = Some(stone);
                Ok(())
            }
            Err(err) => Err(format!("cannot put stone: {}", err)),
        }
    }

    fn can_put(&self, stone: Stone, position: Position) -> Result<(), String> {
        let board_cell = self.get(position.clone());

        // validate position range
        if matches!(board_cell, BoardCell::Wall) {
            Err(format!(
                "the position: {:?} is out of board range",
                position
            ))
        }
        // #################
        // Go rules
        // #################
        // 1. cannot put a stone on the existing stone.
        else if matches!(board_cell, BoardCell::Space(Some(s)) if s == stone) {
            Err(format!(
                "a stone is already on the position: {:?}",
                position
            ))
        } else {
            Ok(())
        }
        // 2. cannot put a stone if the stones connected with it will be killed. but can put when can kill.
    }

    pub fn can_kill(&mut self, stone: Stone, position: Position) -> bool {
        // collect adjacent opponent's stones
        let adjacent_stones: Vec<Position> = vec![
            position.up(),
            position.down(),
            position.left(),
            position.right(),
        ]
        .into_iter()
        .filter_map(|p| match self.get(p.clone()) {
            BoardCell::Space(Some(s)) if s == stone.flip() => Some(p),
            _ => None,
        })
        .collect();

        // stone groups connected with adjacent
        let stone_groups = adjacent_stones
            .into_iter()
            .map(|p| self.get_group(stone, p));

        todo!()
    }

    pub fn kill(&self, stone: Stone, position: Position) -> Result<(), String> {
        // and add them to prisoners
        todo!()
    }

    fn get_group(&mut self, stone: Stone, position: Position) -> Vec<Position> {
        let mut group = vec![];
        let mut memo = HashSet::new();
        let mut stack = vec![position.clone()];

        while let Some(node) = stack.pop() {
            // check if the node is known
            if memo.contains(&node) {
                continue;
            }

            // check if the node's color is same
            let cell = self.get(node.clone());
            if matches!(cell, BoardCell::Space(Some(s)) if s == stone) {
                // add same color to group
                group.push(node.clone());
                // add up,down,left and right of the node to stack to check around the node
                stack.append(&mut vec![node.up(), node.down(), node.left(), node.right()]);
            }
            memo.insert(node.clone());
        }

        group
    }
}

// One origin to express domain
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn up(&self) -> Position {
        Position {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn down(&self) -> Position {
        Position {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(&self) -> Position {
        Position {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(&self) -> Position {
        Position {
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stone::Black => write!(f, "{}", BLACK),
            Stone::White => write!(f, "{}", WHITE),
        }
    }
}

impl TryFrom<String> for Position {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"([0-9]+),([0-9]+)").expect("failed to parse the String to Position");
        for (_, [row, col]) in re.captures_iter(&value).map(|c| c.extract()) {
            return Ok(Self {
                row: row.parse::<i8>().expect("failed to parse number"),
                col: col.parse::<i8>().expect("failed to parse number"),
            });
        }
        Err("failed to parse, Position pattern not found in the String".to_string())
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
                if let Some(stone) = self.space[i][j] {
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
                        write!(f, "{}", "┐ ")?;
                    } else if i == BOARD_SIZE - 1 && j == 0 {
                        write!(f, "└─")?;
                    } else if i == BOARD_SIZE - 1 && j == BOARD_SIZE - 1 {
                        write!(f, "┘ ")?;
                    } else if i == 0 && j != 0 {
                        write!(f, "{}", "┬─")?;
                    } else if i == BOARD_SIZE - 1 && j != 0 {
                        write!(f, "{}", "┴─")?;
                    } else if i != 0 && j == 0 {
                        write!(f, "{}", "├─")?;
                    } else if i != 0 && j == BOARD_SIZE - 1 {
                        write!(f, "{}", "┤ ")?;
                    } else {
                        write!(f, "{}", "┼─")?;
                    }
                }
            }
            // right side line
            write!(f, "{} │\n", NUMBERS[i])?;
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

impl cmp::PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.space == other.space
    }

    fn ne(&self, other: &Board) -> bool {
        self.space != other.space
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_try_from() {
        let given = "0,0".to_string();
        let result = Position::try_from(given).unwrap();
        assert_eq!(result, Position { row: 0, col: 0 });

        let given = "1,0".to_string();
        let result = Position::try_from(given).unwrap();
        assert_eq!(result, Position { row: 1, col: 0 });

        let given = "10,0".to_string();
        let result = Position::try_from(given).unwrap();
        assert_eq!(result, Position { row: 10, col: 0 });

        let given = "10,10".to_string();
        let result = Position::try_from(given).unwrap();
        assert_eq!(result, Position { row: 10, col: 10 });

        // failure case
        let given = "abc".to_string();
        let result = Position::try_from(given);
        assert!(result.is_err());
    }

    #[test]
    fn board_put() {
        let mut board = Board::new();

        let _ = board.put(Stone::Black, Position { row: 1, col: 1 });
        let _ = board.put(Stone::White, Position { row: 1, col: 2 });

        let mut expected = [[None; BOARD_SIZE]; BOARD_SIZE];
        expected[0][0] = Some(Stone::Black);
        expected[0][1] = Some(Stone::White);
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                assert_eq!(board.space[i][j], expected[i][j]);
            }
        }
    }

    #[test]
    fn board_can_put() {
        let mut board = Board::new();

        // ok
        assert!(
            board
                .can_put(Stone::Black, Position { row: 1, col: 1 })
                .is_ok()
        );
        assert!(
            board
                .can_put(Stone::White, Position { row: 1, col: 2 })
                .is_ok()
        );

        // ng
        assert!(
            board
                .can_put(Stone::Black, Position { row: 0, col: 1 })
                .is_err()
        );
        assert!(
            board
                .can_put(Stone::Black, Position { row: 1, col: 0 })
                .is_err()
        );
        assert!(
            board
                .can_put(
                    Stone::Black,
                    Position {
                        row: BOARD_SIZE as i8 + 1,
                        col: 1
                    }
                )
                .is_err()
        );
        assert!(
            board
                .can_put(
                    Stone::Black,
                    Position {
                        row: 1,
                        col: BOARD_SIZE as i8 + 1
                    }
                )
                .is_err()
        );
    }

    // #[test]
    fn board_can_kill() {
        // should kill corner stone
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board
            .put(Stone::White, Position { row: 1, col: 1 })
            .unwrap();
        board
            .put(Stone::Black, Position { row: 1, col: 2 })
            .unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ○ ┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        board
            .put(Stone::Black, Position { row: 2, col: 1 })
            .unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌ ○ ┬─┬─┬─
        // │ ② ○ ┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut expected = Board::new_with_prisoners(0, 1);
        expected
            .put(Stone::Black, Position { row: 1, col: 2 })
            .unwrap();
        expected
            .put(Stone::Black, Position { row: 2, col: 1 })
            .unwrap();
        assert_eq!(board, expected);

        // should kill side stone

        // should kill floated stone

        // should kill multiple stones
    }

    #[test]
    fn board_get_group_of() {
        // no stone should be empty group.
        let mut board = Board::new();
        let group = board.get_group(Stone::Black, Position { row: 1, col: 1 });
        assert_eq!(group, vec![]);

        // single stone should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board
            .put(Stone::White, Position { row: 1, col: 1 })
            .unwrap();
        board
            .put(Stone::Black, Position { row: 1, col: 2 })
            .unwrap();

        let group = board.get_group(Stone::White, Position { row: 1, col: 1 });
        assert_eq!(group, vec![Position { row: 1, col: 1 }]);

        // multiple stones should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ● ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board
            .put(Stone::White, Position { row: 1, col: 1 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 1, col: 2 })
            .unwrap();

        let group = board.get_group(Stone::White, Position { row: 1, col: 1 });
        assert_eq!(
            group,
            vec![Position { row: 1, col: 1 }, Position { row: 1, col: 2 }]
        );

        // and also can refer another position
        let mut group = board.get_group(Stone::White, Position { row: 1, col: 2 });
        group.sort_by_key(|p| p.col);
        assert_eq!(
            group,
            vec![Position { row: 1, col: 1 }, Position { row: 1, col: 2 }]
        );

        // complex stone series should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ● ┬─┬─┬─
        // │ ② ├─● ● ● ●
        // │ ③ ├─┼─● ┼─┼─
        let mut board = Board::new();
        board
            .put(Stone::White, Position { row: 1, col: 1 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 1, col: 2 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 2, col: 2 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 2, col: 3 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 2, col: 4 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 2, col: 5 })
            .unwrap();
        board
            .put(Stone::White, Position { row: 3, col: 3 })
            .unwrap();
        let mut group = board.get_group(Stone::White, Position { row: 1, col: 2 });
        group.sort_by_key(|p| p.col);
        group.sort_by_key(|p| p.row);
        assert_eq!(
            group,
            vec![
                Position { row: 1, col: 1 },
                Position { row: 1, col: 2 },
                Position { row: 2, col: 2 },
                Position { row: 2, col: 3 },
                Position { row: 2, col: 4 },
                Position { row: 2, col: 5 },
                Position { row: 3, col: 3 },
            ]
        )
    }
}
