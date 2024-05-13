// fast ciel(log2(x + 1))
#[inline(always)]
pub fn fast_ceil_log2(x: usize) -> u32 {
    usize::BITS - x.leading_zeros()
}
