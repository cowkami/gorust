use std::fmt;

fn main() {
    let stone = Stone::Black;
    println!("{:}", stone);
    println!("{:}", Stone::White);
}

#[derive(Debug)]
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
