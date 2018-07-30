extern crate client;

use client::board::{Board, Pos};

#[test]
fn board_impl_test() {
    let board = Board::new();

    let s = board.to_string();

    assert_eq!(s.len(), 9 * 8 - 1);

}

#[test]
fn pos_impl_test() {
    match Pos::from_str("A1".to_string()) {
        Ok(x) => {
            assert_eq!("A1".to_string(), x.to_string());
        },
        Err(_) => {
            panic!("Failed to pos(A1)");
        }
    }

    match Pos::from_str("Z9".to_string()) {
        Ok(_) => {
            panic!("Should fail pos(Z9)");
        },
        Err(_) => {
            // nop
            ;
        }
    }
}
