use rand::Rng;
use rand::thread_rng;

use board;
use color;
use pmove;
use player::player::Player;
use super::winnable_solver::get_winnable;

pub struct AI {}

const LAST_SHOT: u32 = 18;
const LAST_SHOT_SIZE: u32 = 64 - LAST_SHOT;


impl AI {
    pub fn new() -> AI {
        AI {}
    }
}



impl Player for AI {
    fn play(&self, board: board::Board, col: color::Color) -> pmove::Move {
        let (black, white) = board.result();

        if LAST_SHOT_SIZE <= (black + white) {
            get_winnable(board, col)
        } else {
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
}
