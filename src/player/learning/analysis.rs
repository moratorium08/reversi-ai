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
            let mut counter = [0u64; $x];
            let mut all_counter = [0u64; $x];
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

    fn diag6(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            print!("{}, ", ((((0.5 + (x as f64)) / ((y as f64) + 1f64)) * (u16::max_value() - 1) as f64)) as u16);
        }
    }

    fn diag7(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn diag8(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn hor_vert2(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn hor_vert3(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn hor_vert4(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn edge2x(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn corner2x5(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }

    fn corner3x3(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];
        self.simulate(|board: board::Board, result: MatchResult| {
            let pat = board.diag4(board::Rotate::Rotate0cw);
            all_counter[pat] += 1;
            if result == MatchResult::Black {
                counter[pat] += 1;
            }
        }
        );

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, ", (x as f64) / (y as f64));
        }
    }
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
