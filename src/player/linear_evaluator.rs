use board;
use player::evaluator::Evaluator;

pub struct Linear {

}

impl Linear {
    pub fn new() -> Linear {
        Linear{}
    }
}

const map: [[i64; 8]; 8]= [
[1000,  -40,   5,   5,   5,   5,  -40, 1000],
[ -40,  -40,   1,   1,   1,   1,  -40, -40],
[   5,    1,   1,   1,   1,   1,    1,   5],
[   5,    1,   1,   1,   1,   1,    1,   5],
[   5,    1,   1,   1,   1,   1,    1,   5],
[   5,    1,   1,   1,   1,   1,    1,   5],
[ -40,  -40,   1,   1,   1,   1,  -40, -40],
[1000,  -40,   5,   5,   5,   5,  -40, 1000],
];

impl Evaluator for Linear {
    fn evaluate(&self, board: board::Board) -> i64 {
        let mut ret = 0i64;
        for x in 0usize..8 {
            for y in 0usize..8 {
                let pos = board::Pos::from_point(x as u8, y as u8);
                match board.color(pos) {
                    Some(color) => {
                        if color.is_black() {
                            ret += map[x][y];
                        } else {
                            ret -= map[x][y];
                        }
                    },
                    None => (),
                }
            }
        }

        ret
    }
}