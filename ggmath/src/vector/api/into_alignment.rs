use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn into_aligned(self) -> Vector<N, T, VecAligned> {
        Vector::from_array(self.into_array())
    }
    #[inline(always)]
    pub fn into_packed(self) -> Vector<N, T, VecPacked> {
        Vector::from_array(self.into_array())
    }
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        Vector::from_array(self.into_array())
    }
}
