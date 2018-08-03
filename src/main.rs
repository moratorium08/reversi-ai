#[macro_use]
extern crate nom;

pub mod board;
pub mod player;
pub mod pmove;
pub mod util;
pub mod interface;
pub mod ui;

use board::{Board, Pos, Hash};
use player::{Player};

fn main() {
    let mut board = Board::new();

    if let Ok(pos) = Pos::from_str("D4".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    }
}
