use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn align(self) -> Rectangle<N, T, VecAligned, R> {
        self.into_alignment()
    }

    #[inline(always)]
    pub fn unalign(self) -> Rectangle<N, T, VecPacked, R> {
        self.into_alignment()
    }
}
