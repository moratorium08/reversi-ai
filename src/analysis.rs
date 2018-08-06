#[macro_use]
extern crate nom;
extern crate colored;


pub mod board;
pub mod color;
pub mod pmove;
pub mod util;
pub mod interface;
pub mod player;


use colored::*;

use player::learning::analysis::gen_statistical_analysis;


const filename: &str = "data/data";
const output_dir: &str = "output/";


fn main() {
    println!("Start analyzing...\n");
    match gen_statistical_analysis(filename, output_dir) {
        Ok(_) => {println!("\n{}", "Finished successfully.".green());},
        Err(s) => {eprintln!("Failed: {}", s);}
    }
}