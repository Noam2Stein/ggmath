use std::mem::transmute;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn get_n_mut<const NOutput: usize>(
        &mut self,
        index: usize,
    ) -> Option<&mut VectorOrScalar<NOutput, T, VecPacked>>
    where
        ScalarCount<NOutput>: VecLenOr1,
    {
        if index >= N - (NOutput - 1) {
            None
        } else {
            Some(unsafe { self.get_n_mut_unchecked(index) })
        }
    }
    #[inline(always)]
    pub fn get_n_n_mut<const NOutput0: usize, const NOutput1: usize>(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
    )>
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
    {
        if indicies[0] >= N - (NOutput0 - 1)
            || indicies[1] >= N - (NOutput1 - 1)
            || range_intersects(indicies[0], NOutput0, indicies[1], NOutput1)
        {
            None
        } else {
            Some(unsafe { self.get_n_n_mut_unchecked::<NOutput0, NOutput1>(indicies) })
        }
    }
    #[inline(always)]
    pub fn get_n_n_n_mut<const NOutput0: usize, const NOutput1: usize, const NOutput2: usize>(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
        &mut VectorOrScalar<NOutput2, T, VecPacked>,
    )>
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
        ScalarCount<NOutput2>: VecLenOr1,
    {
        if indicies[0] >= N - (NOutput0 - 1)
            || indicies[1] >= N - (NOutput1 - 1)
            || indicies[2] >= N - (NOutput2 - 1)
            || range_intersects(indicies[0], NOutput0, indicies[1], NOutput1)
            || range_intersects(indicies[0], NOutput0, indicies[2], NOutput2)
            || range_intersects(indicies[1], NOutput1, indicies[2], NOutput2)
        {
            None
        } else {
            Some(unsafe { self.get_n_n_n_mut_unchecked::<NOutput0, NOutput1, NOutput2>(indicies) })
        }
    }
    #[inline(always)]
    pub fn get_n_n_n_n_mut<
        const NOutput0: usize,
        const NOutput1: usize,
        const NOutput2: usize,
        const NOutput3: usize,
    >(
        &mut self,
        indicies: [usize; 4],
    ) -> Option<(
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
        &mut VectorOrScalar<NOutput2, T, VecPacked>,
        &mut VectorOrScalar<NOutput3, T, VecPacked>,
    )>
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
        ScalarCount<NOutput2>: VecLenOr1,
        ScalarCount<NOutput3>: VecLenOr1,
    {
        if indicies[0] >= N - (NOutput0 - 1)
            || indicies[1] >= N - (NOutput1 - 1)
            || indicies[2] >= N - (NOutput2 - 1)
            || indicies[3] >= N - (NOutput3 - 1)
            || range_intersects(indicies[0], NOutput0, indicies[1], NOutput1)
            || range_intersects(indicies[0], NOutput0, indicies[2], NOutput2)
            || range_intersects(indicies[0], NOutput0, indicies[3], NOutput3)
            || range_intersects(indicies[1], NOutput1, indicies[2], NOutput2)
            || range_intersects(indicies[1], NOutput1, indicies[3], NOutput3)
            || range_intersects(indicies[2], NOutput2, indicies[3], NOutput3)
        {
            None
        } else {
            Some(unsafe {
                self.get_n_n_n_n_mut_unchecked::<NOutput0, NOutput1, NOutput2, NOutput3>(indicies)
            })
        }
    }

    #[inline(always)]
    pub unsafe fn get_n_mut_unchecked<const NOutput: usize>(
        &mut self,
        index: usize,
    ) -> &mut VectorOrScalar<NOutput, T, VecPacked>
    where
        ScalarCount<NOutput>: VecLenOr1,
    {
        transmute(self.as_array().get_unchecked(index))
    }
    #[inline(always)]
    pub unsafe fn get_n_n_mut_unchecked<const NOutput0: usize, const NOutput1: usize>(
        &mut self,
        indicies: [usize; 2],
    ) -> (
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
    )
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
    {
        (
            transmute(self.get_n_mut_unchecked::<NOutput0>(indicies[0])),
            transmute(self.get_n_mut_unchecked::<NOutput1>(indicies[1])),
        )
    }
    #[inline(always)]
    pub unsafe fn get_n_n_n_mut_unchecked<
        const NOutput0: usize,
        const NOutput1: usize,
        const NOutput2: usize,
    >(
        &mut self,
        indicies: [usize; 3],
    ) -> (
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
        &mut VectorOrScalar<NOutput2, T, VecPacked>,
    )
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
        ScalarCount<NOutput2>: VecLenOr1,
    {
        (
            transmute(self.get_n_mut_unchecked::<NOutput0>(indicies[0])),
            transmute(self.get_n_mut_unchecked::<NOutput1>(indicies[1])),
            transmute(self.get_n_mut_unchecked::<NOutput2>(indicies[2])),
        )
    }
    #[inline(always)]
    pub unsafe fn get_n_n_n_n_mut_unchecked<
        const NOutput0: usize,
        const NOutput1: usize,
        const NOutput2: usize,
        const NOutput3: usize,
    >(
        &mut self,
        indicies: [usize; 4],
    ) -> (
        &mut VectorOrScalar<NOutput0, T, VecPacked>,
        &mut VectorOrScalar<NOutput1, T, VecPacked>,
        &mut VectorOrScalar<NOutput2, T, VecPacked>,
        &mut VectorOrScalar<NOutput3, T, VecPacked>,
    )
    where
        ScalarCount<NOutput0>: VecLenOr1,
        ScalarCount<NOutput1>: VecLenOr1,
        ScalarCount<NOutput2>: VecLenOr1,
        ScalarCount<NOutput3>: VecLenOr1,
    {
        (
            transmute(self.get_n_mut_unchecked::<NOutput0>(indicies[0])),
            transmute(self.get_n_mut_unchecked::<NOutput1>(indicies[1])),
            transmute(self.get_n_mut_unchecked::<NOutput2>(indicies[2])),
            transmute(self.get_n_mut_unchecked::<NOutput3>(indicies[3])),
        )
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_n_mut::<1>(index)
    }
    #[inline(always)]
    pub fn get_2_mut(&mut self, index: usize) -> Option<&mut Vector<2, T, VecPacked>> {
        self.get_n_mut::<2>(index)
    }
    #[inline(always)]
    pub fn get_3_mut(&mut self, index: usize) -> Option<&mut Vector<3, T, VecPacked>> {
        self.get_n_mut::<3>(index)
    }
    #[inline(always)]
    pub fn get_4_mut(&mut self, index: usize) -> Option<&mut Vector<4, T, VecPacked>> {
        self.get_n_mut::<4>(index)
    }

    #[inline(always)]
    pub fn get_1_1_mut(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut T)> {
        self.get_n_n_mut::<1, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_1_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_mut::<1, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_1_3_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<3, T, VecPacked>)> {
        self.get_n_n_mut::<1, 3>(indicies)
    }
    #[inline(always)]
    pub fn get_2_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T)> {
        self.get_n_n_mut::<2, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_2_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_mut::<2, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_3_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<3, T, VecPacked>, &mut T)> {
        self.get_n_n_mut::<3, 1>(indicies)
    }

    #[inline(always)]
    pub fn get_1_1_1_mut(&mut self, indicies: [usize; 3]) -> Option<(&mut T, &mut T, &mut T)> {
        self.get_n_n_n_mut::<1, 1, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_1_1_2_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut T, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_n_mut::<1, 1, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_1_2_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>, &mut T)> {
        self.get_n_n_n_mut::<1, 2, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_2_1_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T, &mut T)> {
        self.get_n_n_n_mut::<2, 1, 1>(indicies)
    }

    #[inline(always)]
    pub fn get_1_1_1_1_mut(
        &mut self,
        indicies: [usize; 4],
    ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
        self.get_n_n_n_n_mut::<1, 1, 1, 1>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        self.get_n_mut_unchecked::<1>(index)
    }
    #[inline(always)]
    pub unsafe fn get_2_mut_unchecked(&mut self, index: usize) -> &mut Vector<2, T, VecPacked> {
        self.get_n_mut_unchecked::<2>(index)
    }
    #[inline(always)]
    pub unsafe fn get_3_mut_unchecked(&mut self, index: usize) -> &mut Vector<3, T, VecPacked> {
        self.get_n_mut_unchecked::<3>(index)
    }
    #[inline(always)]
    pub unsafe fn get_4_mut_unchecked(&mut self, index: usize) -> &mut Vector<4, T, VecPacked> {
        self.get_n_mut_unchecked::<4>(index)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_mut_unchecked(&mut self, indicies: [usize; 2]) -> (&mut T, &mut T) {
        self.get_n_n_mut_unchecked::<1, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        self.get_n_n_mut_unchecked::<1, 2>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_3_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<3, T, VecPacked>) {
        self.get_n_n_mut_unchecked::<1, 3>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        self.get_n_n_mut_unchecked::<2, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>) {
        self.get_n_n_mut_unchecked::<2, 2>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut T) {
        self.get_n_n_n_mut_unchecked::<1, 1, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut Vector<2, T, VecPacked>) {
        self.get_n_n_n_mut_unchecked::<1, 1, 2>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>, &mut T) {
        self.get_n_n_n_mut_unchecked::<1, 2, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T, &mut T) {
        self.get_n_n_n_mut_unchecked::<2, 1, 1>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 4],
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        self.get_n_n_n_n_mut_unchecked::<1, 1, 1, 1>(indicies)
    }
}

#[inline(always)]
fn range_intersects(a_start: usize, a_len: usize, b_start: usize, b_len: usize) -> bool {
    a_start < b_start + b_len && b_start < a_start + a_len
}
