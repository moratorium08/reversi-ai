pub mod board;
pub mod util;
pub mod player;

use board::{Board, Pos};
use player::{Player};

fn main() {
    let mut board = Board::new();

    let s = board.to_string();
    println!("{}", s);

    board.print();

    if let Ok(pos) = Pos::from_str("D3".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("C3".to_string()) {
        board = board.flip(&pos, Player::black());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("C4".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("C5".to_string()) {
        board = board.flip(&pos, Player::black());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("B6".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("C6".to_string()) {
        board = board.flip(&pos, Player::black());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("B7".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("C7".to_string()) {
        board = board.flip(&pos, Player::black());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("B5".to_string()) {
        board = board.flip(&pos, Player::white());
        board.print();
    };
    if let Ok(pos) = Pos::from_str("A8".to_string()) {
        board = board.flip(&pos, Player::black());
        board.print();
    };
}
