use board;
use color;
use pmove;

pub trait Player {
    fn play(board: board::Board, player: color::Color) -> pmove::Move;
}