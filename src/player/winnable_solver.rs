use board;
use color;
use pmove;
use player::player::Player;

pub enum MatchResult {
    Win(u8),
    Lose(u8),
    Tie,
}

impl board::BitIndexable for u8 {
    #[inline(always)]
    fn to_index(&self) -> u8 {
        *self
    }
}

#[inline(always)]
fn judge(player: color::Color, black: u32, white: u32) -> MatchResult {
    if black > white {
        let diff = black - white;
        if player.is_black() {
            MatchResult::Win(diff as u8)
        } else {
            MatchResult::Lose(diff as u8)
        }
    } else if white > black {
        let diff = white - black;
        if player.is_white() {
            MatchResult::Win(diff as u8)
        } else {
            MatchResult::Lose(diff as u8)
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
            MatchResult::Win(d) => { return MatchResult::Lose(d); }
            MatchResult::Lose(d) => { return MatchResult::Win(d); }
            MatchResult::Tie => { return MatchResult::Tie; }
        }
    }

    let mut cnt = 0u8;
    let mut current = 64u8;
    let mut status = MatchResult::Lose(64);

    let w = poses;
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
            MatchResult::Lose(d) => { return MatchResult::Win(d); }
            MatchResult::Tie => { status = MatchResult::Tie; }
            MatchResult::Win(d) => {
                /*if current > d {
                    current = d;
                    status = MatchResult::Lose(d);
                }*/
            }
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
    let mut current = 64;

    for pos in poses.iter() {
        let b = board.flip(pos, player);
        match winnable(b, player.opposite(), false) {
            MatchResult::Lose(_) => {
                ret = *pos;
                println!("WIN!");
                return pmove::Move::Mv(*pos);
            }
            MatchResult::Win(d) => {
                if current > d {
                    ret = *pos;
                }
            }
            MatchResult::Tie => {
                ret = *pos;
                current = 0;
            }
        }
    }

    println!("Lose...");
    pmove::Move::Mv(ret)
}

