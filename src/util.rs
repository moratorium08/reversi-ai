// bit utils
#[inline(always)]
pub fn clz(x: u64) -> u8 {
    x.leading_zeros() as u8
}
