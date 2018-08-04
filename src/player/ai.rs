use rand::Rng;
use rand::thread_rng;

use board;
use color;
use pmove;
use player::player::Player;

pub struct AI {}


impl AI {
    pub fn new() -> AI {
        AI{}
    }
}

impl Player for AI {
    fn play(& self, board: board::Board, col: color::Color) -> pmove::Move {
        let f = board.flippable(col);
        let poses = f.poses();
        let len = poses.len();

        let mut rng = thread_rng();

        if len == 0 {
            pmove::Move::Pass
        } else {
            pmove::Move::Mv(poses[rng.gen_range(0, len)])
        }
    }
}
