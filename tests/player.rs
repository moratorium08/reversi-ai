extern crate client;

use client::color::{Color};


#[test]
fn player_impl_test() {
    let p = Color::white ();
    let q = Color::black ();

    assert_eq!(p, q.opposite());
}
