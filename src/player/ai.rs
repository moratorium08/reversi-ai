use rand::Rng;
use rand::thread_rng;

use board;
use color;
use pmove;
use player::player::Player;
use player::evaluator;
use super::winnable_solver::get_winnable;

pub struct AI<T: evaluator::Evaluator> {
    evaluator: T
}

const LAST_SHOT: u32 = 14;
const LAST_SHOT_SIZE: u32 = 64 - LAST_SHOT;

const ALPHA_BETA_DEPTH: u32 = 6;


impl <T: evaluator::Evaluator> AI<T> {
    pub fn new(evaluator: T) -> AI<T> {
        AI {evaluator}
    }
}


impl <T: evaluator::Evaluator> Player for AI<T> {
    fn play(&self, board: board::Board, col: color::Color) -> pmove::Move {
        let (black, white) = board.result();

        if LAST_SHOT_SIZE <= (black + white) {
            get_winnable(board, col)
        } else {
            self.gen_pos_alphabeta(board, col)
        }
    }
}

impl <T: evaluator::Evaluator> AI<T> {
    fn gen_pos_alphabeta(&self, board: board::Board, player: color::Color) -> pmove::Move {
        let f = board.flippable(player);
        let poses = f.poses();
        let len = poses.len();
        if len == 0 {
            pmove::Move::Pass
        } else {
            let mut ret = poses[0];
            let mut current = i64::min_value();
            for pos in poses.iter() {
                let b = board.flip(pos, player);
                let v = -self.alphabeta(b,
                                       player.opposite(),
                                       false,
                                       i64::min_value(),
                                       i64::max_value(),
                                       ALPHA_BETA_DEPTH);
                if current < v {
                    current = v;
                    ret = *pos;
                }
            }
            println!("{}", current);
            pmove::Move::Mv(ret)
        }
    }

    fn alphabeta(&self,
                 board: board::Board,
                 player: color::Color,
                 pass: bool,
                 mut alpha: i64,
                 beta: i64,
                 depth: u32) -> i64 {
        if depth == 0 {
            if player.is_black() {
                return self.evaluator.evaluate(board);
            } else {
                return - self.evaluator.evaluate(board);
            }
        }

        let board::Flippable(mut poses) = board.flippable(player);
        let op = player.opposite();

        if poses == 0 {
            if pass {
                let (black, white)  = board.result();
                if player.is_black() {
                    return (black as i64 - white as i64) * 1000;
                } else {
                    return (white as i64 - black as i64) * 1000;
                }
            }

            return -self.alphabeta(board, op, true, alpha, beta, depth - 1);
        }

        let mut cnt = 0u8;
        while poses > 0 {
            let z = poses.trailing_zeros() as u8;
            if z < 63 {
                poses >>= z + 1;
                cnt += z + 1;
            } else {
                poses = 0;
                cnt = 64;
            }

            let b = board.flip(&(cnt - 1), player);
            let v = -self.alphabeta(b,
                                   op,
                                   false,
                                   -beta,
                                   -alpha,
                                   depth - 1);
            alpha = if alpha < v {v} else {alpha};
            if alpha >= beta {
                return alpha;
            }
        }
        alpha
    }
}
