use board;
use color;
use pmove;

pub trait Player {
    fn play(& self, board: board::Board, player: color::Color) -> pmove::Move;
}