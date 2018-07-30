mod board;

use board::{Board};

fn main() {
    let board = Board::new();

    let s = board.to_string();
    println!("{}", s);
}
