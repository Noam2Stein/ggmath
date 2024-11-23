use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {}
    #[inline(always)]
    pub fn get_2_mut(&mut self, index: usize) -> Option<&mut Vector<2, T, VecPacked>> {}
    #[inline(always)]
    pub fn get_3_mut(&mut self, index: usize) -> Option<&mut Vector<3, T, VecPacked>> {}
    #[inline(always)]
    pub fn get_4_mut(&mut self, index: usize) -> Option<&mut Vector<4, T, VecPacked>> {}

    #[inline(always)]
    pub fn get_1_1_mut(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut T)> {}
    #[inline(always)]
    pub fn get_1_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_1_3_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<3, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_2_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T)> {
    }
    #[inline(always)]
    pub fn get_2_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_3_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<3, T, VecPacked>, &mut T)> {
    }

    #[inline(always)]
    pub fn get_1_1_1_mut(&mut self, indicies: [usize; 3]) -> Option<(&mut T, &mut T, &mut T)> {}
    #[inline(always)]
    pub fn get_1_1_2_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut T, &mut Vector<2, T, VecPacked>)> {
    }
    #[inline(always)]
    pub fn get_1_2_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>, &mut T)> {
    }
    #[inline(always)]
    pub fn get_2_1_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T, &mut T)> {
    }

    #[inline(always)]
    pub fn get_1_1_1_1_mut(
        &mut self,
        indicies: [usize; 4],
    ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
    }

    #[inline(always)]
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {}
    #[inline(always)]
    pub unsafe fn get_2_mut_unchecked(&mut self, index: usize) -> &mut Vector<2, T, VecPacked> {}
    #[inline(always)]
    pub unsafe fn get_3_mut_unchecked(&mut self, index: usize) -> &mut Vector<3, T, VecPacked> {}
    #[inline(always)]
    pub unsafe fn get_4_mut_unchecked(&mut self, index: usize) -> &mut Vector<4, T, VecPacked> {}

    #[inline(always)]
    pub unsafe fn get_1_1_mut_unchecked(&mut self, indicies: [usize; 2]) -> (&mut T, &mut T) {}
    #[inline(always)]
    pub unsafe fn get_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_1_3_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<3, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T) {
    }
    #[inline(always)]
    pub unsafe fn get_2_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>) {
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut T) {
    }
    #[inline(always)]
    pub unsafe fn get_1_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut Vector<2, T, VecPacked>) {
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>, &mut T) {
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T, &mut T) {
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 4],
    ) -> (&mut T, &mut T, &mut T, &mut T) {
    }
}
