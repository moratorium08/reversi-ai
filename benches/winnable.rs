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

#[bench]
fn bench_winnable_2(bench: &mut Bencher) {
    let hash = Hash::from_values(0x0000003258bc3c00, 0x0025ffcda743c3ff);
    let board = Board::from_hash(hash);
    bench.iter(|| {
        get_winnable(board, Color::white());
    });
}

#[bench]
fn bench_winnable_3(bench: &mut Bencher) {
    let hash = Hash::from_values(0xff5d2d303a000e05, 0x0020120f057f2120);
    let board = Board::from_hash(hash);
    bench.iter(|| {
        get_winnable(board, Color::white());
    });
}
