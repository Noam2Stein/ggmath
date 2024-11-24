use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn get_n<const NOutput: usize>(self, index: usize) -> Option<VectorOrScalar<NOutput, T, A>>
    where
        ScalarCount<NOutput>: VecLenOr1,
    {
    }

    #[inline(always)]
    pub fn get(self, index: usize) -> Option<T> {}
    #[inline(always)]
    pub fn get_2(self, index: usize) -> Option<Vector<2, T, A>> {}
    #[inline(always)]
    pub fn get_3(self, index: usize) -> Option<Vector<3, T, A>> {}
    #[inline(always)]
    pub fn get_4(self, index: usize) -> Option<Vector<4, T, A>> {}

    #[inline(always)]
    pub fn get_1_1(self, indicies: [usize; 2]) -> Option<Vector<2, T, A>> {}
    #[inline(always)]
    pub fn get_1_2(self, indicies: [usize; 2]) -> Option<Vector<3, T, A>> {}
    #[inline(always)]
    pub fn get_1_3(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {}
    #[inline(always)]
    pub fn get_2_1(self, indicies: [usize; 2]) -> Option<Vector<3, T, A>> {}
    #[inline(always)]
    pub fn get_2_2(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {}
    #[inline(always)]
    pub fn get_3_1(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {}

    #[inline(always)]
    pub fn get_1_1_1(self, indicies: [usize; 3]) -> Option<Vector<3, T, A>> {}
    #[inline(always)]
    pub fn get_1_1_2(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {}
    #[inline(always)]
    pub fn get_1_2_1(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {}
    #[inline(always)]
    pub fn get_2_1_1(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {}

    #[inline(always)]
    pub fn get_1_1_1_1(self, indicies: [usize; 4]) -> Option<Vector<4, T, A>> {}

    #[inline(always)]
    pub unsafe fn get_unchecked(self, index: usize) -> T {}
    #[inline(always)]
    pub unsafe fn get_2_unchecked(self, index: usize) -> Vector<2, T, A> {}
    #[inline(always)]
    pub unsafe fn get_3_unchecked(self, index: usize) -> Vector<3, T, A> {}
    #[inline(always)]
    pub unsafe fn get_4_unchecked(self, index: usize) -> Vector<4, T, A> {}

    #[inline(always)]
    pub unsafe fn get_1_1_unchecked(self, indicies: [usize; 2]) -> Vector<2, T, A> {}
    #[inline(always)]
    pub unsafe fn get_1_2_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {}
    #[inline(always)]
    pub unsafe fn get_1_3_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {}
    #[inline(always)]
    pub unsafe fn get_2_1_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {}
    #[inline(always)]
    pub unsafe fn get_2_2_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {}

    #[inline(always)]
    pub unsafe fn get_1_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<3, T, A> {}
    #[inline(always)]
    pub unsafe fn get_1_1_2_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {}
    #[inline(always)]
    pub unsafe fn get_1_2_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {}
    #[inline(always)]
    pub unsafe fn get_2_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {}

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_unchecked(self, indicies: [usize; 4]) -> Vector<4, T, A> {}
}
