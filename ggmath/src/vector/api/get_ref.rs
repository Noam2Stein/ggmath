use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn get_ref(&self, index: usize) -> Option<&T> {}
    #[inline(always)]
    pub fn get_2_ref(&self, index: usize) -> Option<&Vector<2, T, VecPacked>> {}
    #[inline(always)]
    pub fn get_3_ref(&self, index: usize) -> Option<&Vector<3, T, VecPacked>> {}
    #[inline(always)]
    pub fn get_4_ref(&self, index: usize) -> Option<&Vector<4, T, VecPacked>> {}

    #[inline(always)]
    pub fn get_1_1_ref(&self, indicies: [usize; 2]) -> Option<(&T, &T)> {}
    #[inline(always)]
    pub fn get_1_2_ref(&self, indicies: [usize; 2]) -> Option<(&T, &Vector<2, T, VecPacked>)> {}
    #[inline(always)]
    pub fn get_1_3_ref(&self, indicies: [usize; 2]) -> Option<(&T, &Vector<3, T, VecPacked>)> {}
    #[inline(always)]
    pub fn get_2_1_ref(&self, indicies: [usize; 2]) -> Option<(&Vector<2, T, VecPacked>, &T)> {}
    #[inline(always)]
    pub fn get_2_2_ref(
        &self,
        indicies: [usize; 2],
    ) -> Option<(&Vector<2, T, VecPacked>, &Vector<2, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_3_1_ref(&self, indicies: [usize; 2]) -> Option<(&Vector<3, T, VecPacked>, &T)> {}

    #[inline(always)]
    pub fn get_1_1_1_ref(&self, indicies: [usize; 3]) -> Option<(&T, &T, &T)> {}
    #[inline(always)]
    pub fn get_1_1_2_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&T, &T, &Vector<2, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_1_2_1_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&T, &Vector<2, T, VecPacked>, &T)> {
    }
    #[inline(always)]
    pub fn get_2_1_1_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&Vector<2, T, VecPacked>, &T, &T)> {
    }

    #[inline(always)]
    pub fn get_1_1_1_1_ref(&self, indicies: [usize; 4]) -> Option<(&T, &T, &T, &T)> {}

    #[inline(always)]
    pub unsafe fn get_ref_unchecked(&self, index: usize) -> &T {}
    #[inline(always)]
    pub unsafe fn get_2_ref_unchecked(&self, index: usize) -> &Vector<2, T, VecPacked> {}
    #[inline(always)]
    pub unsafe fn get_3_ref_unchecked(&self, index: usize) -> &Vector<3, T, VecPacked> {}
    #[inline(always)]
    pub unsafe fn get_4_ref_unchecked(&self, index: usize) -> &Vector<4, T, VecPacked> {}

    #[inline(always)]
    pub unsafe fn get_1_1_ref_unchecked(&self, indicies: [usize; 2]) -> (&T, &T) {}
    #[inline(always)]
    pub unsafe fn get_1_2_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&T, &Vector<2, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_1_3_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&T, &Vector<3, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_2_1_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&Vector<2, T, VecPacked>, &T) {
    }
    #[inline(always)]
    pub unsafe fn get_2_2_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&Vector<2, T, VecPacked>, &Vector<2, T, VecPacked>) {
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_ref_unchecked(&self, indicies: [usize; 3]) -> (&T, &T, &T) {}
    #[inline(always)]
    pub unsafe fn get_1_1_2_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&T, &T, &Vector<2, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&T, &Vector<2, T, VecPacked>, &T) {
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&Vector<2, T, VecPacked>, &T, &T) {
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_ref_unchecked(&self, indicies: [usize; 4]) -> (&T, &T, &T, &T) {}
}
