pub mod board;
pub mod util;
pub mod player;
pub mod interface;

use board::{Board, Pos, Hash};
use player::{Player};

fn main() {
    let mut board = Board::new();

    if let Ok(pos) = Pos::from_str("D4".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    }
}
