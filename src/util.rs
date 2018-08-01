// bit utils
pub fn clz(x: u64) -> u8 {
    let mut mask = 1u64 << 63;
    let mut cnt = 0u8;
    while mask > 0 {
        if mask & x != 0 {
            return cnt;
        }
        cnt += 1;
        mask >>= 1;
    }
    63
}
