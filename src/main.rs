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
    let table = [
        ("D3", 17729692631040u64),
        ("C3", 17181179904u64),
        ("C4", 17729692237824u64),
        ("C5", 17180917760u64),
        ("B6", 68178344542720u64),
        ("C6", 4407176468480u64),
        ("B7", 624531228197376u64),
        ("C7", 1424967607521280u64),
        ("B5", 578774270830838272u64),
        ("A8", 72340172841156608u64),
    ];

    let mut board = Board::new();
    let mut player = Player::black();

    for &(s, v) in table.iter() {
        if let Ok(pos) = Pos::from_str(s.to_string()) {
            board.print_player_board(player);
            println!("{:?}", board.flippable(player));
            board = board.flip(&pos, player);
            player = player.opposite();
        } else {
            panic!("Failed to put : {}", s);
        }
    }
    board.print_player_board(player);
    println!("{:?}", board.flippable(player));
}
