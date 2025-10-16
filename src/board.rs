use regex::Regex;
use std::{collections::HashSet, fmt};

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
    space: Space,
    pub black_prisoners: usize,
    pub white_prisoners: usize,
    previous_spaces: Vec<Space>,
}

type Space = [[Option<Stone>; BOARD_SIZE]; BOARD_SIZE];

impl Board {
    pub fn new() -> Self {
        let space = [[None; BOARD_SIZE]; BOARD_SIZE];
        Self {
            space,
            black_prisoners: 0,
            white_prisoners: 0,
            previous_spaces: vec![space],
        }
    }

    pub fn new_with_prisoners(black_prisoners: usize, white_prisoners: usize) -> Self {
        let mut board = Self::new();
        board.black_prisoners += black_prisoners;
        board.white_prisoners += white_prisoners;
        board
    }

    fn get(&self, point: Point) -> BoardCell {
        // check point range
        if point.row <= 0
            || BOARD_SIZE < point.row as usize
            || point.col <= 0
            || BOARD_SIZE < point.col as usize
        {
            BoardCell::Wall
        } else {
            BoardCell::Space(self.space[point.row as usize - 1][point.col as usize - 1])
        }
    }

    pub fn put(&mut self, stone: Stone, point: Point) -> Result<(), String> {
        // validate for go rule
        match self.can_put(stone, point) {
            Ok(_) => {
                self.kill_by(stone, point);
                self.space[point.row as usize - 1][point.col as usize - 1] = Some(stone);
                self.previous_spaces.push(self.space.clone());
                Ok(())
            }
            Err(err) => Err(format!("cannot put stone: {}", err)),
        }
    }

    pub fn find_available_points(&mut self, stone: Stone) -> Vec<Point> {
        let mut available_points = vec![];
        for row in 1..=BOARD_SIZE {
            for col in 1..=BOARD_SIZE {
                let point = Point {
                    row: row as i8,
                    col: col as i8,
                };
                if self.can_put(stone, point).is_ok() {
                    available_points.push(point);
                }
            }
        }
        available_points
    }

    fn can_put(&mut self, stone: Stone, point: Point) -> Result<(), String> {
        let board_cell = self.get(point);
        // validate point range
        if matches!(board_cell, BoardCell::Wall) {
            Err(format!("the point: {:?} is out of board range", point))
        }
        // #################
        // Go rules
        // #################
        // cannot put a stone on the existing stone.
        else if matches!(board_cell, BoardCell::Space(Some(_))) {
            Err(format!("a stone is already on the point: {:?}", point))
        // cannot put a stone if the stones connected with it will be killed. but can put when can kill.
        } else if self.is_suicide(stone, point)
            && self.find_groups_can_kill(stone, point).len() == 0
        {
            Err(format!("it's a suicide move."))
        }
        // cannot put a stone make same space as the last space.
        // ko is occurred.
        else if self.is_same_last_space(stone, point) {
            Err(format!("cannot take KO at {:?} for {:?}", point, stone))
        } else {
            Ok(())
        }
    }

    fn kill_by(&mut self, stone: Stone, point: Point) {
        let groups = self.find_groups_can_kill(stone, point);
        if groups.len() == 0 {
            return;
        }
        // remove all groups
        for group in groups.iter() {
            for p in group.iter() {
                self.space[p.row as usize - 1][p.col as usize - 1] = None;
            }
            // add numbers of group to prisoners
            match stone {
                Stone::Black => {
                    self.black_prisoners += group.len();
                }
                Stone::White => {
                    self.white_prisoners += group.len();
                }
            }
        }
    }

    fn find_groups_can_kill(&self, stone: Stone, point: Point) -> Vec<Vec<Point>> {
        // todo: refacter not to use unwrap
        vec![
            // find opponent's stone from around
            point.up(),
            point.down(),
            point.left(),
            point.right(),
        ]
        .into_iter()
        .filter_map(|p| match self.get(p) {
            // choose opponent's stones
            BoardCell::Space(Some(s)) if s == stone.flip() => Some(p),
            _ => None,
        })
        // find groups of opponent's stones
        .map(|p| self.find_group(stone.flip(), p))
        // choose group that breathing space is 1 and given point
        .filter_map(|g| {
            let breathing_space = self.find_breathing_space(g.clone());
            if breathing_space.len() == 1 && breathing_space[0] == point {
                Some(g)
            } else {
                None
            }
        })
        .collect()
    }

    fn find_breathing_space(&self, group: Vec<Point>) -> Vec<Point> {
        // todo: refoctor not to use unwrap
        if group.len() == 0 {
            return vec![];
        }
        let mut breathing_points = vec![];
        let mut checked_points = HashSet::new();
        let mut check_points = vec![];
        for p in group.into_iter() {
            // group points are all opponent's color stones, so no need to check.
            checked_points.insert(p);
            check_points.append(&mut vec![p.up(), p.down(), p.right(), p.left()]);
        }
        // check around the point
        while let Some(check_point) = check_points.pop() {
            // is check_point already checked?
            if checked_points.contains(&check_point) {
                continue;
            }
            // if the cell is empty
            if matches!(self.get(check_point), BoardCell::Space(None)) {
                breathing_points.push(check_point);
                checked_points.insert(check_point);
            }
        }
        breathing_points
    }

    fn find_group(&self, stone: Stone, start_point: Point) -> Vec<Point> {
        let mut group = vec![];
        let mut checked_points = HashSet::new();
        let mut check_points = vec![start_point];

        while let Some(point) = check_points.pop() {
            // check if the node is known
            if checked_points.contains(&point) {
                continue;
            }
            // check if the node's color is same
            if matches!(
                self.get(point),
                BoardCell::Space(Some(s)) if s == stone,
            ) {
                // add same color to group
                group.push(point);
                // add up,down,left and right of the node to stack to check around the node
                check_points.append(&mut vec![
                    point.up(),
                    point.down(),
                    point.left(),
                    point.right(),
                ]);
            }
            checked_points.insert(point);
        }

        group
    }

    fn is_suicide(&mut self, stone: Stone, point: Point) -> bool {
        // todo: refactor? implementing temporary put method is very considerable.
        // put stone temporary
        self.space[point.row as usize - 1][point.col as usize - 1] = Some(stone);
        // calculate breathing space of put stone
        let group = self.find_group(stone, point);
        let breathing_space = self.find_breathing_space(group);
        // remove stone put temporary
        self.space[point.row as usize - 1][point.col as usize - 1] = None;
        breathing_space.len() == 0
    }

    fn is_same_last_space(&mut self, stone: Stone, point: Point) -> bool {
        let history_length = self.previous_spaces.len();
        if history_length < 4 {
            return false;
        }
        // todo: refactor? implementing temporary put method is very considerable.
        // put stone temporary
        let mut temp_board = Self::new();
        temp_board.space = self.space;
        temp_board.kill_by(stone, point);
        temp_board.space[point.row as usize - 1][point.col as usize - 1] = Some(stone);
        // check
        temp_board.space == self.previous_spaces[history_length - 2]
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.space == other.space
            && self.black_prisoners == other.black_prisoners
            && self.white_prisoners == other.white_prisoners
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// One origin to express domain
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    // 1-origin
    pub row: i8,
    pub col: i8,
}

impl Point {
    pub fn up(&self) -> Point {
        Point {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn down(&self) -> Point {
        Point {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(&self) -> Point {
        Point {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(&self) -> Point {
        Point {
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

impl TryFrom<String> for Point {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"([0-9]+),([0-9]+)").expect("failed to parse the String to Point");
        for (_, [row, col]) in re.captures_iter(&value).map(|c| c.extract()) {
            return Ok(Self {
                row: row.parse::<i8>().expect("failed to parse number"),
                col: col.parse::<i8>().expect("failed to parse number"),
            });
        }
        Err("failed to parse, Point pattern not found in the String".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_try_from() {
        let given = "0,0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { row: 0, col: 0 });

        let given = "1,0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { row: 1, col: 0 });

        let given = "10,0".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { row: 10, col: 0 });

        let given = "10,10".to_string();
        let result = Point::try_from(given).unwrap();
        assert_eq!(result, Point { row: 10, col: 10 });

        // failure case
        let given = "abc".to_string();
        let result = Point::try_from(given);
        assert!(result.is_err());
    }

    #[test]
    fn board_put() {
        let mut board = Board::new();

        let _ = board.put(Stone::Black, Point { row: 1, col: 1 });
        let _ = board.put(Stone::White, Point { row: 1, col: 2 });

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
        // ok
        let mut board = Board::new();
        assert!(
            board
                .can_put(Stone::Black, Point { row: 1, col: 1 })
                .is_ok()
        );
        assert!(
            board
                .can_put(Stone::White, Point { row: 1, col: 2 })
                .is_ok()
        );

        // ng
        let mut board = Board::new();
        assert!(
            board
                .can_put(Stone::Black, Point { row: 0, col: 1 })
                .is_err()
        );
        assert!(
            board
                .can_put(Stone::Black, Point { row: 1, col: 0 })
                .is_err()
        );
        assert!(
            board
                .can_put(
                    Stone::Black,
                    Point {
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
                    Point {
                        row: 1,
                        col: BOARD_SIZE as i8 + 1
                    }
                )
                .is_err()
        );

        board.put(Stone::White, Point { row: 1, col: 1 }).unwrap();
        assert!(
            board
                .can_put(Stone::White, Point { row: 1, col: 1 })
                .is_err()
        );
        assert!(
            board
                .can_put(Stone::Black, Point { row: 1, col: 1 })
                .is_err()
        );

        // ok
        // when can kill, can put regardless suicide move.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌─┬─○ ● ┬─
        // │ ② ├─○ ● ┼─●
        // │ ③ ├─┼─○ ● ┼─
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 3 }).unwrap();

        board.put(Stone::White, Point { row: 1, col: 4 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 3 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 5 }).unwrap();
        board.put(Stone::White, Point { row: 3, col: 4 }).unwrap();
        assert!(
            board
                .can_put(Stone::Black, Point { row: 2, col: 4 })
                .is_ok()
        );
    }

    #[test]
    fn board_kill() {
        // should kill corner stone
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board.put(Stone::White, Point { row: 1, col: 1 }).unwrap();
        board.put(Stone::Black, Point { row: 1, col: 2 }).unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ○ ┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        board.put(Stone::Black, Point { row: 2, col: 1 }).unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌ ○ ┬─┬─┬─
        // │ ② ○ ┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut expected = Board::new_with_prisoners(1, 0);
        expected
            .put(Stone::Black, Point { row: 1, col: 2 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 1 })
            .unwrap();
        assert_eq!(board, expected);

        // should kill side stone
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌ ○ ● ┬─┬─
        // │ ② ├─┼─○ ┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 2 }).unwrap();
        board.put(Stone::White, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 3 }).unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌ ○ ● ○ ┬─
        // │ ② ├─┼─○ ┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        board.put(Stone::Black, Point { row: 1, col: 4 }).unwrap();

        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌ ○ ┬─○ ┬─
        // │ ② ├─┼─○ ┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut expected = Board::new_with_prisoners(1, 0);
        expected
            .put(Stone::Black, Point { row: 1, col: 2 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 3 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 1, col: 4 })
            .unwrap();
        assert_eq!(board, expected);

        // should kill floated stone
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌─┬─○ ┬─┬─
        // │ ② ├─○ ● ○ ┼─
        // │ ③ ├─┼─○ ┼─┼─
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 4 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 3 }).unwrap();

        let mut expected = Board::new_with_prisoners(1, 0);
        expected
            .put(Stone::Black, Point { row: 1, col: 3 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 2 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 4 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 3, col: 3 })
            .unwrap();
        assert_eq!(board, expected);

        // should kill multiple stones
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌─┬─○ ○─┬─
        // │ ② ├─○ ● ● ○─
        // │ ③ ├─┼─○ ○─┼─
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 1, col: 4 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 3 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 4 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 5 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 4 }).unwrap();

        let mut expected = Board::new_with_prisoners(2, 0);
        expected
            .put(Stone::Black, Point { row: 1, col: 3 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 1, col: 4 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 2 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 2, col: 5 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 3, col: 3 })
            .unwrap();
        expected
            .put(Stone::Black, Point { row: 3, col: 4 })
            .unwrap();
        assert_eq!(board, expected);
    }

    #[test]
    fn board_find_group() {
        // no stone should be empty group.
        let board = Board::new();
        let group = board.find_group(Stone::Black, Point { row: 1, col: 1 });
        assert_eq!(group, vec![]);

        // single stone should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ○ ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board.put(Stone::White, Point { row: 1, col: 1 }).unwrap();
        board.put(Stone::Black, Point { row: 1, col: 2 }).unwrap();

        let group = board.find_group(Stone::White, Point { row: 1, col: 1 });
        assert_eq!(group, vec![Point { row: 1, col: 1 }]);

        // multiple stones should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ● ┬─┬─┬─
        // │ ② ├─┼─┼─┼─┼─
        // │ ③ ├─┼─┼─┼─┼─
        let mut board = Board::new();
        board.put(Stone::White, Point { row: 1, col: 1 }).unwrap();
        board.put(Stone::White, Point { row: 1, col: 2 }).unwrap();

        let group = board.find_group(Stone::White, Point { row: 1, col: 1 });
        assert_eq!(
            group,
            vec![Point { row: 1, col: 1 }, Point { row: 1, col: 2 }]
        );

        // and also can refer another point
        let mut group = board.find_group(Stone::White, Point { row: 1, col: 2 });
        group.sort_by_key(|p| p.col);
        assert_eq!(
            group,
            vec![Point { row: 1, col: 1 }, Point { row: 1, col: 2 }]
        );

        // complex stone series should be group.
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ● ● ┬─┬─┬─
        // │ ② ├─● ● ● ●
        // │ ③ ├─┼─● ┼─┼─
        let mut board = Board::new();
        board.put(Stone::White, Point { row: 1, col: 1 }).unwrap();
        board.put(Stone::White, Point { row: 1, col: 2 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 3 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 4 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 5 }).unwrap();
        board.put(Stone::White, Point { row: 3, col: 3 }).unwrap();
        let mut group = board.find_group(Stone::White, Point { row: 1, col: 2 });
        group.sort_by_key(|p| p.col);
        group.sort_by_key(|p| p.row);
        assert_eq!(
            group,
            vec![
                Point { row: 1, col: 1 },
                Point { row: 1, col: 2 },
                Point { row: 2, col: 2 },
                Point { row: 2, col: 3 },
                Point { row: 2, col: 4 },
                Point { row: 2, col: 5 },
                Point { row: 3, col: 3 },
            ]
        )
    }

    #[test]
    fn test_is_suicide() {
        // ┌─────────────
        // │   ① ② ③ ④
        // │ ① ┌─┬─○ ┬─┬─
        // │ ② ├─○ ┼─○ ┼─
        // │ ③ ├─┼─○ ┼─┼─
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 4 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 3 }).unwrap();
        assert!(
            board
                .can_put(Stone::White, Point { row: 2, col: 3 })
                .is_err()
        );
    }

    #[test]
    fn test_can_take_ko() {
        // ┌─────────────
        // │    ①②③④⑤⑥
        // │ ① ┌┬○●┬┬
        // │ ② ├○┼○●┼
        // │ ③ ├┼○●┼┼
        let mut board = Board::new();
        board.put(Stone::Black, Point { row: 1, col: 3 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 2 }).unwrap();
        board.put(Stone::Black, Point { row: 2, col: 4 }).unwrap();
        board.put(Stone::Black, Point { row: 3, col: 3 }).unwrap();
        board.put(Stone::White, Point { row: 1, col: 4 }).unwrap();
        board.put(Stone::White, Point { row: 2, col: 5 }).unwrap();
        board.put(Stone::White, Point { row: 3, col: 4 }).unwrap();

        // ┌─────────────
        // │    ①②③④⑤⑥
        // │ ① ┌┬○●┬┬
        // │ ② ├○●┼●┼
        // │ ③ ├┼○●┼┼
        board.put(Stone::White, Point { row: 2, col: 3 }).unwrap();

        // this is prohibited
        assert!(
            board
                .can_put(Stone::Black, Point { row: 2, col: 4 })
                .is_err()
        );

        // this is ok because black done KO threat.
        // ┌─────────────
        // │    ①②③④⑤⑥
        // │ ① ○┬○●┬┬
        // │ ② ├○●┼●┼
        // │ ③ ├┼○●┼●
        board.put(Stone::Black, Point { row: 1, col: 1 }).unwrap();
        board.put(Stone::White, Point { row: 3, col: 6 }).unwrap();
        assert!(
            board
                .can_put(Stone::Black, Point { row: 2, col: 4 })
                .is_ok()
        );
    }
}
