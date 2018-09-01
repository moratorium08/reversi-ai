use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::{File, create_dir};
use std::path::Path;
use std::vec::Vec;

use colored::*;

use board;
use color;
use player::learning::parser::{parse, MatchResult, History, Pos};
use player::linear_evaluator as eval;

const FEATURE_DUMP: &str = "train.csv";

struct Analyzer<'a> {
    output_dir: &'a str,
    history: Vec<History>,
}

macro_rules! gen_analyzer {
    ($name:ident, $x:expr, $name_str: expr) => {
        fn $name(&self) {
            let mut counter = [0u32; $x];
            let mut all_counter = [0u32; $x];
            let board = self.simulate(|board: board::Board, result: MatchResult, epoch: usize| {
                if epoch < 12 || epoch > 18 {
                    return;
                }
                let pat = board.$name(board::Rotate::Rotate0cw);
                all_counter[pat] += 1;
                if result == MatchResult::Black {
                    counter[pat] += 1;
                }
            }
            );
        print!("pub const {}: [u16; {}] = [", $name_str, $x);
        let mut greater_than_100 = 0u64;
        for i in 0..$x{
            let x = counter[i];
            let y = all_counter[i];
            if all_counter[i] > 5 {
                greater_than_100 += 1;
                print!("{}, ", ((((0.5 + (x as f64)) / ((y as f64) + 1f64)) * (u16::max_value() - 1) as f64)) as u16);
            } else {
                print!("{}, ", u16::max_value() / 2);
            }
        }
        println!("];");
        eprintln!("greater: {}", greater_than_100);
        }
    }
}


impl<'a> Analyzer<'a> {
    fn new(mut reader: BufReader<File>, output_dir: &'a str) -> Analyzer {
        let mut v = Vec::<History>::new();
        for line in reader.lines() {
            // ここメモリコピーしたくないなあ
            let line = line.unwrap();
            match parse(&(line.to_string() + "\n")) {
                Ok(("", h)) => {
                    v.push(h);
                }
                _ => {
                    eprintln!("{}: parse: {}", "[Failed]".red(), line);
                }
            }
        }
        Analyzer { history: v, output_dir }
    }

    fn modify_history(&mut self) {
        let mut new_his = Vec::<History>::new();
        for his in self.history.iter() {
            let mut board = board::Board::new();
            let mut player = color::Color::black();
            let mut epoch = 0u8;

            for pos in his.poses.iter() {
                match pos {
                    Pos::Pass => (),
                    Pos::Mv(p) => {
                        board = board.flip(p, player);
                    }
                }
                player = player.opposite();
                epoch += 1;
            }

            let (black, white) = board.result();
            let result = if black > white {
                MatchResult::Black
            }
            else if white > black {
                MatchResult::White
            } else {
                MatchResult::Draw
            };
            let mut new_vec = Vec::<Pos>::new();
            for p in his.poses.iter() {
                new_vec.push(*p);
            }
            if (black + white) == 64 {
                if (white > black && his.result == MatchResult::White) ||
                   (black > white && his.result == MatchResult::Black) {
                    new_his.push(History{result, poses:new_vec})
                }
            }
        }
        self.history = new_his;
    }


    fn analyze(&self) {
        // board analysis
        self.diag4();
        self.diag5();
        self.diag6();
        self.diag7();
        self.diag8();
        self.hor_vert2();
        self.hor_vert3();
        self.hor_vert4();
        self.edge2x();
        self.corner2x5();
        self.corner3x3();
        self.corner2x2();


        // feature dump
        self.dump_feature();
    }

    fn dump_feature(&self) {
        let path = Path::new(self.output_dir);
        if !path.exists() {
            // error handling
            create_dir(path);
        }
        let file = path.join(FEATURE_DUMP);

        let mut writer = match File::create(file) {
            Ok(f) => {
                BufWriter::new(f)
            }
            Err(_) => panic!("Failed to open dump"),
        };

        for his in self.history.iter() {
            // 引き分けは今は解析しない
            if his.result == MatchResult::Draw {
                continue;
            }

            let mut board = board::Board::new();
            let mut player = color::Color::black();
            let mut epoch = 0u8;
            let x = 0..1;

            for pos in his.poses.iter() {
                epoch += 1;
                if epoch < 12 {
                    continue;
                }
                match pos {
                    Pos::Pass => (),
                    Pos::Mv(p) => {
                        board = board.flip(p, player);
                        write!(&mut writer, "{}", if his.result == MatchResult::Black{1}else{0});
                        let features = eval::calculate_features(board);
                        for item in features.iter() {
                            write!(&mut writer, ",{:.3}", item);
                        }
                        write!(&mut writer, "\n");
                    }
                }
                player = player.opposite();
                if epoch >= 18 {
                    break;
                }
            }
        }
    }

    fn simulate<F>(&self, mut update: F) where F: FnMut(board::Board, MatchResult, usize) {
        for his in self.history.iter() {
            // 引き分けは今は解析しない
            if his.result == MatchResult::Draw {
                continue;
            }

            let mut board = board::Board::new();
            let mut player = color::Color::black();
            let mut epoch = 0u8;

            const ROTATES: [board::Rotate; 4] = [
                board::Rotate::Rotate0cw,
                board::Rotate::Rotate90cw,
                board::Rotate::Rotate180cw,
                board::Rotate::Rotate270cw];

            for (i, pos) in his.poses.iter().enumerate() {
                match pos {
                    Pos::Pass => (),
                    Pos::Mv(p) => {
                        board = board.flip(p, player);
                        for rotate in ROTATES.iter() {
                            update(board.rotate(*rotate), his.result, i);
                        }
                    }
                }
                player = player.opposite();
                epoch += 1;
            }
            let (black, white) = board.result();
            if (black + white) != 64 {
                eprintln!("Not end");
            }
            if black > white {
                if MatchResult::Black !=  his.result {
                    eprintln!("Match result is inconsistent");
                }
            }
            else if black < white {
                if MatchResult::White !=  his.result {
                    eprintln!("Match result is inconsistent");
                }
            }
        }
    }

    gen_analyzer!(diag4, 81,  "DIAG4");
    gen_analyzer!(diag5, 243, "DIAG5");
    gen_analyzer!(diag6, 729, "DIAG6");
    gen_analyzer!(diag7, 2187, "DIAG7");
    gen_analyzer!(diag8, 6561, "DIAG8");
    gen_analyzer!(hor_vert2, 6561, "HOR_VERT2");
    gen_analyzer!(hor_vert3, 6561, "HOR_VERT3");
    gen_analyzer!(hor_vert4, 6561, "HOR_VERT4");
    gen_analyzer!(edge2x, 59049, "EDGE2X");
    gen_analyzer!(corner2x5, 59049, "CORNER2X5");
    gen_analyzer!(corner3x3, 19683, "CORNER3X3");
    gen_analyzer!(corner2x2, 81, "CORNER2X2");
}

pub fn gen_statistical_analysis(filename: &str, output_dir: &str) -> Result<(), &'static str> {
    match File::open(filename) {
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut analyzer = Analyzer::new(reader, output_dir);
            analyzer.modify_history();
            analyzer.analyze();
            Ok(())
        }
        Err(_) => Err("Failed to open"),
    }
}

