use board;
use player;
use pmove;

pub trait UI {
    fn play(board: board::Board, player: player::Player) -> pmove::Move;
}