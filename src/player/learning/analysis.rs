use std::io::{BufReader, BufRead};
use std::fs::File;
use std::vec::Vec;

use colored::*;

use board;
use color;
use player::learning::parser::{parse, MatchResult, History, Pos};

struct Analyzer<'a> {
    output_dir: &'a str,
    history: Vec<History>,
}

macro_rules! gen_analyzer{
    ($name:ident, $x:expr, $name_str: expr) => {
        fn $name(&self) {
            let mut counter = [0u32; $x];
            let mut all_counter = [0u32; $x];
            self.simulate(|board: board::Board, result: MatchResult| {
                let pat = board.$name(board::Rotate::Rotate0cw);
                all_counter[pat] += 1;
                if result == MatchResult::Black {
                    counter[pat] += 1;
                }
            }
            );
        print!("const {} [u16; {}] = [", $name_str, $x);
        for i in 0..$x{
            let x = counter[i];
            let y = all_counter[i];
            print!("{}, ", ((((0.5 + (x as f64)) / ((y as f64) + 1f64)) * (u16::max_value() - 1) as f64)) as u16);
        }
        println!("];");
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

    fn analyze(&self) {
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

    }

    fn simulate<F>(&self, mut update: F) where F: FnMut(board::Board, MatchResult) -> () {
        for his in self.history.iter() {
            // 引き分けは今は解析しない
            if his.result == MatchResult::Draw {
                continue;
            }

            let mut board = board::Board::new();
            let mut player = color::Color::black();
            let mut epoch = 0u8;
            let x = 0..1;

            const ROTATES: [board::Rotate; 4] = [
                board::Rotate::Rotate0cw,
                board::Rotate::Rotate90cw,
                board::Rotate::Rotate180cw,
                board::Rotate::Rotate270cw];

            for pos in his.poses.iter() {
                match pos {
                    Pos::Pass => (),
                    Pos::Mv(p) => {
                        board = board.flip(p, player);
                        for rotate in ROTATES.iter() {
                            update(board.rotate(*rotate), his.result);
                        }
                    }
                }
                player = player.opposite();
                epoch += 1;
                if epoch >= 50 {
                    break;
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
    gen_analyzer!(corner3x3, 19683, "CORNER3x4");
}

pub fn gen_statistical_analysis(filename: &str, output_dir: &str) -> Result<(), &'static str> {
    match File::open(filename) {
        Ok(f) => {
            let reader = BufReader::new(f);
            let analyzer = Analyzer::new(reader, output_dir);
            analyzer.analyze();
            Ok(())
        }
        Err(_) => Err("Failed to open"),
    }
}
