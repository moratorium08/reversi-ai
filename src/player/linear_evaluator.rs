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

const map1: [[i64; 8]; 8] = [
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
        let features = calculate_features(board);
        let coef = [0.87322733, -0.24674064, 0.45119731, -0.55724615, 0.33250983, -0.2325938,
        0.36340727,  0.26275155,  -1.51238341, -1.14778382, -0.40514061,  0.04111813];
        let mut ret = 0i64;
        for i in 0..12 {
            ret += (features[i] * coef[i] as f32 * 100f32) as i64;
        }
        ret
    }
}

fn calc(x: u16) -> f32 {
    let x = (x as f32) / ((u16::max_value() - 1) as f32) - 0.5f32;
    if (x < 0f32 && -x < 0.1) || (x > 0f32 && x < 0.1) {
        0.0f32
    } else {
        x
    }
}

macro_rules! gen_feature {
    ($name:ident, $array:ident) => {
        pub fn $name(board: board::Board) -> f32 {
            calc(param::$array[board.$name(board::Rotate::Rotate0cw)]) +
            calc(param::$array[board.$name(board::Rotate::Rotate90cw)]) +
            calc(param::$array[board.$name(board::Rotate::Rotate180cw)]) +
            calc(param::$array[board.$name(board::Rotate::Rotate270cw)])
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
pub const feature_names: [&str; 12] = [
        "diag4", "diag5", "diag6", "diag7", "diag8", "hor_vert2", "hor_vert3", "hor_vert4",
        "edge2x", "corner2x2", "corner3x3", "flippable_diff"
];

pub fn print_features(board: board::Board) {
    let features = calculate_features(board);
    for i in 0..12 {
        println!("{}: {}", feature_names[i], features[i]);
    }
}

pub fn calculate_features(board: board::Board) -> Vec<f32> {
    let mut v = Vec::<f32>::new();

    for f in feature_functions.iter() {
        v.push(f(board));
    }
    v
}

