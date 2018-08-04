// bit utils
#[inline(always)]
pub fn clz(x: u64) -> u8 {
    // できればあとで64のときどうにかしたい
    x.leading_zeros() as u8
}
