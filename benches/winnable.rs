#![feature(test)]

extern crate client;
extern crate test;


use test::Bencher;

use client::board::{Board, Hash};
use client::color::Color;
use client::player::winnable_solver::get_winnable;



#[bench]
fn bench_winnable_1(bench: &mut Bencher) {
    let hash = Hash::from_values(0x8ecfee3728150000, 0x50201088542a572e);
    let board = Board::from_hash(hash);
    bench.iter(|| {
        get_winnable(board, Color::white());
    });
}
