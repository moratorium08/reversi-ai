extern crate client;


use client::board;
use client::color;
use client::player::winnable_solver::{MatchResult, winnable};

#[test]
fn test_winnable() {
    let h = board::Hash::from_values(0x0000044c5e3e0300, 0xfffcf8b0a0c0c0e7);
    let board = board::Board::from_hash(h);
    match winnable(board, color::Color::black(), false) {
        MatchResult::Win(_) => (),
        _ => panic!("Failed: winnable")
    };
}

