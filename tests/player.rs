extern crate client;

use client::player::{Player};


#[test]
fn player_impl_test() {
    let p = Player::white ();
    let q = Player::black ();

    assert_eq!(p, q.opposite());
}
