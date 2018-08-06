use std::vec::Vec;

use board;
use color;
use player::evaluator::Evaluator;
use player::param;

pub struct Linear {}

impl Linear {
    pub fn new() -> Linear {
        Linear {}
    }
}

const map: [[i64; 8]; 8] = [
    [1000, -40, 5, 5, 5, 5, -40, 1000],
    [-40, -40, 1, 1, 1, 1, -40, -40],
    [5, 1, 1, 1, 1, 1, 1, 5],
    [5, 1, 1, 1, 1, 1, 1, 5],
    [5, 1, 1, 1, 1, 1, 1, 5],
    [5, 1, 1, 1, 1, 1, 1, 5],
    [-40, -40, 1, 1, 1, 1, -40, -40],
    [1000, -40, 5, 5, 5, 5, -40, 1000],
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
                    }
                    None => (),
                }
            }
        }
        let board::Flippable(b) = board.flippable(color::Color::black());
        let board::Flippable(c) = board.flippable(color::Color::white());

        let point = b.count_ones() as i64 - c.count_ones() as i64;

        ret + point * 10
    }
}

macro_rules! gen_feature {
    ($name:ident, $array:ident) => {
        pub fn $name(board: board::Board) -> f32 {
            ((param::$array[board.$name(board::Rotate::Rotate0cw)] as f32) / ((u16::max_value() - 1) as f32) - 0.5f32) +
            ((param::$array[board.$name(board::Rotate::Rotate90cw)] as f32) / ((u16::max_value() - 1) as f32) - 0.5f32) +
            ((param::$array[board.$name(board::Rotate::Rotate180cw)] as f32) / ((u16::max_value() - 1) as f32) - 0.5f32) +
            ((param::$array[board.$name(board::Rotate::Rotate270cw)] as f32) / ((u16::max_value() - 1) as f32) - 0.5f32)
        }
    }
}

fn flippable_diff(board: board::Board) -> f32 {
    let board::Flippable(b) = board.flippable(color::Color::black());
    let board::Flippable(c) = board.flippable(color::Color::white());

    let point = b.count_ones() as i64 - c.count_ones() as i64;
    point as f32
}

gen_feature!(diag4, DIAG4);
gen_feature!(diag5, DIAG5);
gen_feature!(diag6, DIAG6);
gen_feature!(diag7, DIAG7);
gen_feature!(diag8, DIAG8);
gen_feature!(hor_vert2,  HOR_VERT2);
gen_feature!(hor_vert3,  HOR_VERT3);
gen_feature!(hor_vert4,  HOR_VERT4);
gen_feature!(edge2x, EDGE2X);
gen_feature!(corner2x5, CORNER2X5);
gen_feature!(corner3x3, CORNER3X3);
gen_feature!(corner2x2, CORNER2X2);


pub const feature_functions: [fn(board::Board) -> f32; 12] = [
            diag4, diag5, diag6, diag7, diag8, hor_vert2, hor_vert3, hor_vert4,
            edge2x, corner2x2, corner3x3, flippable_diff
        ];

pub fn calculate_features(board: board::Board) -> Vec<f32> {
    let mut v = Vec::<f32>::new();

    for f in feature_functions.iter() {
        v.push(f(board));
    }
    v
}

