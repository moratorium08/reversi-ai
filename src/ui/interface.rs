use board;
use player;
use pmove;

trait Interface {
    fn game(board: board::Board, player: player::Play) -> pmove::Move;
}