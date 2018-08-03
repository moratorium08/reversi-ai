extern crate client;

use client::board::{Board, Hash, Pos, Flippable};
use client::player::Player;

#[test]
fn board_impl_test() {
    let board = Board::new();

    let s = board.to_string();

    assert_eq!(s.len(), 9 * 8 - 1);

    assert_eq!(board, Board::from_hash(board.hash()));

    let hash = Hash::from_values(0x0000000810000000, 0x0000001008000000);
    assert_eq!(hash, board.hash());
    assert_eq!(Board::from_hash(hash), board);
}

#[test]
fn board_flip_test() {
    let table = [
        ("D3", 0x0000001000000000, 0x0000000818080000),
        ("C3", 0x0000001008040000, 0x0000000810080000),
        ("C4", 0x0000001000040000, 0x000000081c080000),
        ("C5", 0x0000001c04040000, 0x0000000018080000),
        ("B6", 0x0000001804040000, 0x0000020418080000),
        ("C6", 0x0000041c04040000, 0x0000020018080000),
        ("B7", 0x0000001404040000, 0x0002060818080000),
        ("C7", 0x0004041404040000, 0x0002020818080000),
        ("B5", 0x0004041000040000, 0x0002020e1c080000),
        ("A8", 0x0106041000040000, 0x0000020e1c080000)
    ];

    let mut board = Board::new();
    let mut player = Player::black();

    for &(s, black, white) in table.iter() {
        if let Ok(pos) = Pos::from_str(s.to_string()) {
            board = board.flip(&pos, player);
            let hash = Hash::from_values(white, black);
            assert_eq!(board.hash(), hash);
            player = player.opposite();
        } else {
            panic!("Failed to put : {}", s);
        }
    }
}

#[test]
fn board_flippable_test() {
    let table = [
        ("D3", 17729692631040u64),
        ("C3", 17181179904u64),
        ("C4", 17729692237824u64),
        ("C5", 17180917760u64),
        ("B6", 68178344542720u64),
        ("C6", 4407176468480u64),
        ("B7", 624531228197376u64),
        ("C7", 1424967607521280u64),
        ("B5", 578774270830838272u64),
        ("A8", 72340172841156608u64),
    ];
    let finally = 1011119826434917888u64;

    let mut board = Board::new();
    let mut player = Player::black();

    for &(s, v) in table.iter() {
        if let Ok(pos) = Pos::from_str(s.to_string()) {
            assert_eq!(Flippable(v), board.flippable(player));
            board = board.flip(&pos, player);
            player = player.opposite();
        } else {
            panic!("Failed to put : {}", s);
        }
    }
    assert_eq!(Flippable(finally), board.flippable(player));
}

#[test]
fn pos_impl_test() {
    match Pos::from_str("A1".to_string()) {
        Ok(x) => {
            assert_eq!("A1".to_string(), x.to_string());
        }
        Err(_) => {
            panic!("Failed to pos(A1)");
        }
    }

    match Pos::from_str("Z9".to_string()) {
        Ok(_) => {
            panic!("Should fail pos(Z9)");
        }
        Err(_) => {
            // nop
            ;
        }
    }
}
