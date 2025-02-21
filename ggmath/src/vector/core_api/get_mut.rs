use std::mem::transmute;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub const fn get_n_mut<const N_OUTPUT: usize>(
        &mut self,
        index: usize,
    ) -> Option<&mut VectorOrT<N_OUTPUT, T, VecPacked>>
    where
        ScalarCount<N_OUTPUT>: VecLenOr1,
    {
        if index >= N - (N_OUTPUT - 1) {
            None
        } else {
            Some(unsafe { self.get_n_mut_unchecked(index) })
        }
    }
    #[inline(always)]
    pub const fn get_n_n_mut<const N_OUTPUT_0: usize, const N_OUTPUT_1: usize>(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
    {
        if indicies[0] >= N - (N_OUTPUT_0 - 1)
            || indicies[1] >= N - (N_OUTPUT_1 - 1)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[1], N_OUTPUT_1)
        {
            None
        } else {
            Some(unsafe { self.get_n_n_mut_unchecked::<N_OUTPUT_0, N_OUTPUT_1>(indicies) })
        }
    }
    #[inline(always)]
    pub const fn get_n_n_n_mut<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
    >(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_2, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
    {
        if indicies[0] >= N - (N_OUTPUT_0 - 1)
            || indicies[1] >= N - (N_OUTPUT_1 - 1)
            || indicies[2] >= N - (N_OUTPUT_2 - 1)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[1], N_OUTPUT_1)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[2], N_OUTPUT_2)
            || range_intersects(indicies[1], N_OUTPUT_1, indicies[2], N_OUTPUT_2)
        {
            None
        } else {
            Some(unsafe {
                self.get_n_n_n_mut_unchecked::<N_OUTPUT_0, N_OUTPUT_1, N_OUTPUT_2>(indicies)
            })
        }
    }
    #[inline(always)]
    pub const fn get_n_n_n_n_mut<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
        const N_OUTPUT_3: usize,
    >(
        &mut self,
        indicies: [usize; 4],
    ) -> Option<(
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_2, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_3, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
        ScalarCount<N_OUTPUT_3>: VecLenOr1,
    {
        if indicies[0] >= N - (N_OUTPUT_0 - 1)
            || indicies[1] >= N - (N_OUTPUT_1 - 1)
            || indicies[2] >= N - (N_OUTPUT_2 - 1)
            || indicies[3] >= N - (N_OUTPUT_3 - 1)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[1], N_OUTPUT_1)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[2], N_OUTPUT_2)
            || range_intersects(indicies[0], N_OUTPUT_0, indicies[3], N_OUTPUT_3)
            || range_intersects(indicies[1], N_OUTPUT_1, indicies[2], N_OUTPUT_2)
            || range_intersects(indicies[1], N_OUTPUT_1, indicies[3], N_OUTPUT_3)
            || range_intersects(indicies[2], N_OUTPUT_2, indicies[3], N_OUTPUT_3)
        {
            None
        } else {
            Some(unsafe {
                self.get_n_n_n_n_mut_unchecked::<N_OUTPUT_0, N_OUTPUT_1, N_OUTPUT_2, N_OUTPUT_3>(
                    indicies,
                )
            })
        }
    }

    #[inline(always)]
    pub const unsafe fn get_n_mut_unchecked<const N_OUTPUT: usize>(
        &mut self,
        index: usize,
    ) -> &mut VectorOrT<N_OUTPUT, T, VecPacked>
    where
        ScalarCount<N_OUTPUT>: VecLenOr1,
    {
        unsafe { transmute(self.as_array_mut().as_mut_ptr().add(index).as_mut()) }
    }
    #[inline(always)]
    pub const unsafe fn get_n_n_mut_unchecked<const N_OUTPUT_0: usize, const N_OUTPUT_1: usize>(
        &mut self,
        indicies: [usize; 2],
    ) -> (
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
    {
        unsafe {
            (
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_0>(indicies[0])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_1>(indicies[1])),
            )
        }
    }
    #[inline(always)]
    pub const unsafe fn get_n_n_n_mut_unchecked<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
    >(
        &mut self,
        indicies: [usize; 3],
    ) -> (
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_2, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
    {
        unsafe {
            (
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_0>(indicies[0])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_1>(indicies[1])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_2>(indicies[2])),
            )
        }
    }
    #[inline(always)]
    pub const unsafe fn get_n_n_n_n_mut_unchecked<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
        const N_OUTPUT_3: usize,
    >(
        &mut self,
        indicies: [usize; 4],
    ) -> (
        &mut VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_2, T, VecPacked>,
        &mut VectorOrT<N_OUTPUT_3, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
        ScalarCount<N_OUTPUT_3>: VecLenOr1,
    {
        unsafe {
            (
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_0>(indicies[0])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_1>(indicies[1])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_2>(indicies[2])),
                transmute(self.get_n_mut_unchecked::<N_OUTPUT_3>(indicies[3])),
            )
        }
    }

    #[inline(always)]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_n_mut::<1>(index)
    }
    #[inline(always)]
    pub const fn get_2_mut(&mut self, index: usize) -> Option<&mut Vector<2, T, VecPacked>> {
        self.get_n_mut::<2>(index)
    }
    #[inline(always)]
    pub const fn get_3_mut(&mut self, index: usize) -> Option<&mut Vector<3, T, VecPacked>> {
        self.get_n_mut::<3>(index)
    }
    #[inline(always)]
    pub const fn get_4_mut(&mut self, index: usize) -> Option<&mut Vector<4, T, VecPacked>> {
        self.get_n_mut::<4>(index)
    }

    #[inline(always)]
    pub const fn get_1_1_mut(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut T)> {
        self.get_n_n_mut::<1, 1>(indicies)
    }
    #[inline(always)]
    pub const fn get_1_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_mut::<1, 2>(indicies)
    }
    #[inline(always)]
    pub const fn get_1_3_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut T, &mut Vector<3, T, VecPacked>)> {
        self.get_n_n_mut::<1, 3>(indicies)
    }
    #[inline(always)]
    pub const fn get_2_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T)> {
        self.get_n_n_mut::<2, 1>(indicies)
    }
    #[inline(always)]
    pub const fn get_2_2_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_mut::<2, 2>(indicies)
    }
    #[inline(always)]
    pub const fn get_3_1_mut(
        &mut self,
        indicies: [usize; 2],
    ) -> Option<(&mut Vector<3, T, VecPacked>, &mut T)> {
        self.get_n_n_mut::<3, 1>(indicies)
    }

    #[inline(always)]
    pub const fn get_1_1_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut T, &mut T)> {
        self.get_n_n_n_mut::<1, 1, 1>(indicies)
    }
    #[inline(always)]
    pub const fn get_1_1_2_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut T, &mut Vector<2, T, VecPacked>)> {
        self.get_n_n_n_mut::<1, 1, 2>(indicies)
    }
    #[inline(always)]
    pub const fn get_1_2_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut T, &mut Vector<2, T, VecPacked>, &mut T)> {
        self.get_n_n_n_mut::<1, 2, 1>(indicies)
    }
    #[inline(always)]
    pub const fn get_2_1_1_mut(
        &mut self,
        indicies: [usize; 3],
    ) -> Option<(&mut Vector<2, T, VecPacked>, &mut T, &mut T)> {
        self.get_n_n_n_mut::<2, 1, 1>(indicies)
    }

    #[inline(always)]
    pub const fn get_1_1_1_1_mut(
        &mut self,
        indicies: [usize; 4],
    ) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
        self.get_n_n_n_n_mut::<1, 1, 1, 1>(indicies)
    }

    #[inline(always)]
    pub const unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        unsafe { self.get_n_mut_unchecked::<1>(index) }
    }
    #[inline(always)]
    pub const unsafe fn get_2_mut_unchecked(
        &mut self,
        index: usize,
    ) -> &mut Vector<2, T, VecPacked> {
        unsafe { self.get_n_mut_unchecked::<2>(index) }
    }
    #[inline(always)]
    pub const unsafe fn get_3_mut_unchecked(
        &mut self,
        index: usize,
    ) -> &mut Vector<3, T, VecPacked> {
        unsafe { self.get_n_mut_unchecked::<3>(index) }
    }
    #[inline(always)]
    pub const unsafe fn get_4_mut_unchecked(
        &mut self,
        index: usize,
    ) -> &mut Vector<4, T, VecPacked> {
        unsafe { self.get_n_mut_unchecked::<4>(index) }
    }

    #[inline(always)]
    pub const unsafe fn get_1_1_mut_unchecked(&mut self, indicies: [usize; 2]) -> (&mut T, &mut T) {
        unsafe { self.get_n_n_mut_unchecked::<1, 1>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        unsafe { self.get_n_n_mut_unchecked::<1, 2>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_1_3_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut T, &mut Vector<3, T, VecPacked>) {
        unsafe { self.get_n_n_mut_unchecked::<1, 3>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        unsafe { self.get_n_n_mut_unchecked::<2, 1>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_2_2_mut_unchecked(
        &mut self,
        indicies: [usize; 2],
    ) -> (&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>) {
        unsafe { self.get_n_n_mut_unchecked::<2, 2>(indicies) }
    }

    #[inline(always)]
    pub const unsafe fn get_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut T) {
        unsafe { self.get_n_n_n_mut_unchecked::<1, 1, 1>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_1_1_2_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut T, &mut Vector<2, T, VecPacked>) {
        unsafe { self.get_n_n_n_mut_unchecked::<1, 1, 2>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_1_2_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut T, &mut Vector<2, T, VecPacked>, &mut T) {
        unsafe { self.get_n_n_n_mut_unchecked::<1, 2, 1>(indicies) }
    }
    #[inline(always)]
    pub const unsafe fn get_2_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 3],
    ) -> (&mut Vector<2, T, VecPacked>, &mut T, &mut T) {
        unsafe { self.get_n_n_n_mut_unchecked::<2, 1, 1>(indicies) }
    }

    #[inline(always)]
    pub const unsafe fn get_1_1_1_1_mut_unchecked(
        &mut self,
        indicies: [usize; 4],
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        unsafe { self.get_n_n_n_n_mut_unchecked::<1, 1, 1, 1>(indicies) }
    }
}

#[inline(always)]
const fn range_intersects(a_start: usize, a_len: usize, b_start: usize, b_len: usize) -> bool {
    a_start < b_start + b_len && b_start < a_start + a_len
}
