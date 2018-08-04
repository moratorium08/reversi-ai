extern crate client;

use client::util::{clz};

#[test]
fn board_bit_functions() {
    assert_eq!(clz(0xffbbccddaau64), 24);
    // This is not defined.
    // assert_eq!(clz(0), 63);
    assert_eq!(clz(1), 63);
    assert_eq!(clz(1 << 63), 0);
}
