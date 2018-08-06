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


impl <'a>Analyzer <'a>{
    fn new(mut reader: BufReader<File>, output_dir: &'a str) -> Analyzer {
        let mut v = Vec::<History>::new();
        for line in reader.lines() {
            // ここメモリコピーしたくないなあ
            let line = line.unwrap();
            match parse(&(line.to_string() + "\n")) {
                Ok(("", h)) => {
                    v.push(h);
                },
                _ => {
                    eprintln!("{}: parse: {}", "[Failed]".red(), line);
                }

            }
        }
        Analyzer{history: v, output_dir}
    }

    fn analyze(&self) {
        self.diag4();
    }

    fn diag4(&self) {
        let mut counter = [0u64; 81];
        let mut all_counter = [0u64; 81];

        let mut black_win = 0u64;
        let mut white_win = 0u64;

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
                            let pat = board.diag4(*rotate);
                            all_counter[pat] += 1;
                            if his.result == MatchResult::Black {
                                counter[pat] += 1;
                                black_win += 1;
                            } else {
                                white_win += 1;
                            }
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

        for i in 0..81 {
            let x = counter[i];
            let y = all_counter[i];

            println!("{}, {}, {}", x, y, (x as f64) / (y as f64));
        }
        println!("Black win{}", black_win);
        println!("White win{}", white_win);
    }
}

pub fn gen_statistical_analysis(filename: &str, output_dir: &str) -> Result<(), &'static str> {
    match File::open(filename) {
        Ok(f) => {
            let reader = BufReader::new(f);
            let analyzer = Analyzer::new(reader, output_dir);
            analyzer.analyze();
            Ok(())
        },
        Err(_) => Err("Failed to open"),
    }
}
