use board;
use color;
use pmove;

pub enum MatchResult {
    Win,
    Lose,
    Tie,
}

impl board::BitIndexable for u8 {
    #[inline(always)]
    fn to_index(&self) -> u8 {
        *self
    }
}

#[inline(always)]
fn judge(player: color::Color, black: u8, white: u8) -> MatchResult {
    if black > white {
        if player.is_black() {
            MatchResult::Win
        } else {
            MatchResult::Lose
        }
    } else if white > black {
        if player.is_white() {
            MatchResult::Win
        } else {
            MatchResult::Lose
        }
    } else {
        MatchResult::Tie
    }
}


pub fn winnable(board: board::Board, player: color::Color, pass: bool) -> MatchResult {
    let (black, white) = board.result();
    if black + white == 64 {
        return judge(player, black, white);
    }

    let board::Flippable(mut poses) = board.flippable(player);

    let op = player.opposite();
    if poses == 0 {
        if pass {
            return judge(player, black, white);
        }
        match winnable(board, op, true) {
            MatchResult::Win => { return MatchResult::Lose; }
            MatchResult::Lose => { return MatchResult::Win; }
            MatchResult::Tie => { return MatchResult::Tie; }
        }
    }

    let mut cnt = 0u8;
    let mut status = MatchResult::Lose;

    while poses > 0 {
        let z = poses.trailing_zeros() as u8;
        if z < 63 {
            poses >>= z + 1;
            cnt += z + 1;
        } else {
            poses = 0;
            cnt = 64;
        }

        let r = winnable(board.flip(&(cnt - 1), player), op, false);
        match r {
            MatchResult::Lose => { return MatchResult::Win; }
            MatchResult::Tie => { status = MatchResult::Tie; }
            MatchResult::Win =>
                (),
        }
    }

    status
}

pub fn get_winnable(board: board::Board, player: color::Color) -> pmove::Move {
    let f = board.flippable(player);
    let poses = f.poses();

    if poses.len() == 0 {
        println!("passed");
        return pmove::Move::Pass;
    }

    let mut ret: board::Pos = poses[0];

    let l = poses.len();
    println!("Size: {}", l);
    for pos in poses.iter() {
        println!("* {}...", pos.to_string());
        let b = board.flip(pos, player);
        match winnable(b, player.opposite(), false) {
            MatchResult::Lose => {
                println!("WIN!");
                return pmove::Move::Mv(*pos);
            }
            MatchResult::Win => {
            }
            MatchResult::Tie => {
                ret = *pos;
            }
        }
    }

    println!("Lose...");
    pmove::Move::Mv(ret)
}

