use super::*;

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn align(self) -> Aabb<N, T, VecAligned, R> {
        self.to_layout()
    }

    #[inline(always)]
    pub fn unalign(self) -> Aabb<N, T, VecPacked, R> {
        self.to_layout()
    }
}
