#[macro_use]
extern crate nom;
extern crate colored;
extern crate rand;

pub mod board;
pub mod color;
pub mod pmove;
pub mod util;
pub mod interface;
pub mod player;


use interface::game;
use interface::tcp_client;
use player::ai;
use player::linear_evaluator;


const HOST: &str = "localhost";
const PORT: u16 = 3000;
const NAME: &str = "mike";

fn main() {

    let client = tcp_client::ClientBuilder::new()
        .host(HOST)
        .port(PORT)
        .finalize()
        .unwrap();


    let player = ai::AI::new(linear_evaluator::Linear::new());

    match game::Game::new(client, player, NAME) {
        Ok(mut g) => g.main_loop(),
        Err(s) => panic!("Fail: {}", s),
    };
}
