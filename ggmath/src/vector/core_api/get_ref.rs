use std::mem::transmute;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn get_n_ref<const N_OUTPUT: usize>(
        &self,
        index: usize,
    ) -> Option<&VectorOrT<N_OUTPUT, T, VecPacked>>
    where
        ScalarCount<N_OUTPUT>: VecLenOr1,
    {
        if index >= N - (N_OUTPUT - 1) {
            None
        } else {
            Some(unsafe { self.get_n_ref_unchecked(index) })
        }
    }
    #[inline(always)]
    pub fn get_n_n_ref<const N_OUTPUT_0: usize, const N_OUTPUT_1: usize>(
        &self,
        indicies: [usize; 2],
    ) -> Option<(
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
    {
        if let Some(ref_0) = self.get_n_ref::<N_OUTPUT_0>(indicies[0]) {
            if let Some(ref_1) = self.get_n_ref::<N_OUTPUT_1>(indicies[1]) {
                Some((ref_0, ref_1))
            } else {
                None
            }
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn get_n_n_n_ref<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
    >(
        &self,
        indicies: [usize; 3],
    ) -> Option<(
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &VectorOrT<N_OUTPUT_2, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
    {
        if let Some(ref_0) = self.get_n_ref::<N_OUTPUT_0>(indicies[0]) {
            if let Some(ref_1) = self.get_n_ref::<N_OUTPUT_1>(indicies[1]) {
                if let Some(ref_2) = self.get_n_ref::<N_OUTPUT_2>(indicies[2]) {
                    Some((ref_0, ref_1, ref_2))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn get_n_n_n_n_ref<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
        const N_OUTPUT_3: usize,
    >(
        &self,
        indicies: [usize; 4],
    ) -> Option<(
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &VectorOrT<N_OUTPUT_2, T, VecPacked>,
        &VectorOrT<N_OUTPUT_3, T, VecPacked>,
    )>
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
        ScalarCount<N_OUTPUT_3>: VecLenOr1,
    {
        if let Some(ref_0) = self.get_n_ref::<N_OUTPUT_0>(indicies[0]) {
            if let Some(ref_1) = self.get_n_ref::<N_OUTPUT_1>(indicies[1]) {
                if let Some(ref_2) = self.get_n_ref::<N_OUTPUT_2>(indicies[2]) {
                    if let Some(ref_3) = self.get_n_ref::<N_OUTPUT_3>(indicies[3]) {
                        Some((ref_0, ref_1, ref_2, ref_3))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline(always)]
    pub unsafe fn get_n_ref_unchecked<const N_OUTPUT: usize>(
        &self,
        index: usize,
    ) -> &VectorOrT<N_OUTPUT, T, VecPacked>
    where
        ScalarCount<N_OUTPUT>: VecLenOr1,
    {
        transmute(self.as_array().get_unchecked(index))
    }
    #[inline(always)]
    pub unsafe fn get_n_n_ref_unchecked<const N_OUTPUT_0: usize, const N_OUTPUT_1: usize>(
        &self,
        indicies: [usize; 2],
    ) -> (
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
    {
        (
            self.get_n_ref_unchecked::<N_OUTPUT_0>(indicies[0]),
            self.get_n_ref_unchecked::<N_OUTPUT_1>(indicies[1]),
        )
    }
    #[inline(always)]
    pub unsafe fn get_n_n_n_ref_unchecked<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
    >(
        &self,
        indicies: [usize; 3],
    ) -> (
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &VectorOrT<N_OUTPUT_2, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
    {
        (
            self.get_n_ref_unchecked::<N_OUTPUT_0>(indicies[0]),
            self.get_n_ref_unchecked::<N_OUTPUT_1>(indicies[1]),
            self.get_n_ref_unchecked::<N_OUTPUT_2>(indicies[2]),
        )
    }
    #[inline(always)]
    pub unsafe fn get_n_n_n_n_ref_unchecked<
        const N_OUTPUT_0: usize,
        const N_OUTPUT_1: usize,
        const N_OUTPUT_2: usize,
        const N_OUTPUT_3: usize,
    >(
        &self,
        indicies: [usize; 4],
    ) -> (
        &VectorOrT<N_OUTPUT_0, T, VecPacked>,
        &VectorOrT<N_OUTPUT_1, T, VecPacked>,
        &VectorOrT<N_OUTPUT_2, T, VecPacked>,
        &VectorOrT<N_OUTPUT_3, T, VecPacked>,
    )
    where
        ScalarCount<N_OUTPUT_0>: VecLenOr1,
        ScalarCount<N_OUTPUT_1>: VecLenOr1,
        ScalarCount<N_OUTPUT_2>: VecLenOr1,
        ScalarCount<N_OUTPUT_3>: VecLenOr1,
    {
        (
            self.get_n_ref_unchecked::<N_OUTPUT_0>(indicies[0]),
            self.get_n_ref_unchecked::<N_OUTPUT_1>(indicies[1]),
            self.get_n_ref_unchecked::<N_OUTPUT_2>(indicies[2]),
            self.get_n_ref_unchecked::<N_OUTPUT_3>(indicies[3]),
        )
    }

    #[inline(always)]
    pub fn get_ref(&self, index: usize) -> Option<&T> {
        self.get_n_ref::<1>(index)
    }
    #[inline(always)]
    pub fn get_2_ref(&self, index: usize) -> Option<&Vector<2, T, VecPacked>> {
        self.get_n_ref::<2>(index)
    }
    #[inline(always)]
    pub fn get_3_ref(&self, index: usize) -> Option<&Vector<3, T, VecPacked>> {
        self.get_n_ref::<3>(index)
    }
    #[inline(always)]
    pub fn get_4_ref(&self, index: usize) -> Option<&Vector<4, T, VecPacked>> {
        self.get_n_ref::<4>(index)
    }

    #[inline(always)]
    pub fn get_1_1_ref(&self, indicies: [usize; 2]) -> Option<(&T, &T)> {
        self.get_n_n_ref::<1, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_1_2_ref(&self, indicies: [usize; 2]) -> Option<(&T, &Vector<2, T, VecPacked>)> {
        self.get_n_n_ref::<1, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_1_3_ref(&self, indicies: [usize; 2]) -> Option<(&T, &Vector<3, T, VecPacked>)> {
        self.get_n_n_ref::<1, 3>(indicies)
    }
    #[inline(always)]
    pub fn get_2_1_ref(&self, indicies: [usize; 2]) -> Option<(&Vector<2, T, VecPacked>, &T)> {
        self.get_n_n_ref::<2, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_2_2_ref(
        &self,
        indicies: [usize; 2],
    ) -> Option<(&Vector<2, T, VecPacked>, &Vector<2, T, VecPacked>)> {
        self.get_n_n_ref::<2, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_3_1_ref(&self, indicies: [usize; 2]) -> Option<(&Vector<3, T, VecPacked>, &T)> {
        self.get_n_n_ref::<3, 1>(indicies)
    }

    #[inline(always)]
    pub fn get_1_1_1_ref(&self, indicies: [usize; 3]) -> Option<(&T, &T, &T)> {
        self.get_n_n_n_ref::<1, 1, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_1_1_2_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&T, &T, &Vector<2, T, VecPacked>)> {
        self.get_n_n_n_ref::<1, 1, 2>(indicies)
    }
    #[inline(always)]
    pub fn get_1_2_1_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&T, &Vector<2, T, VecPacked>, &T)> {
        self.get_n_n_n_ref::<1, 2, 1>(indicies)
    }
    #[inline(always)]
    pub fn get_2_1_1_ref(
        &self,
        indicies: [usize; 3],
    ) -> Option<(&Vector<2, T, VecPacked>, &T, &T)> {
        self.get_n_n_n_ref::<2, 1, 1>(indicies)
    }

    #[inline(always)]
    pub fn get_1_1_1_1_ref(&self, indicies: [usize; 4]) -> Option<(&T, &T, &T, &T)> {
        self.get_n_n_n_n_ref::<1, 1, 1, 1>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_ref_unchecked(&self, index: usize) -> &T {
        self.get_n_ref_unchecked::<1>(index)
    }
    #[inline(always)]
    pub unsafe fn get_2_ref_unchecked(&self, index: usize) -> &Vector<2, T, VecPacked> {
        self.get_n_ref_unchecked::<2>(index)
    }
    #[inline(always)]
    pub unsafe fn get_3_ref_unchecked(&self, index: usize) -> &Vector<3, T, VecPacked> {
        self.get_n_ref_unchecked::<3>(index)
    }
    #[inline(always)]
    pub unsafe fn get_4_ref_unchecked(&self, index: usize) -> &Vector<4, T, VecPacked> {
        self.get_n_ref_unchecked::<4>(index)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_ref_unchecked(&self, indicies: [usize; 2]) -> (&T, &T) {
        self.get_n_n_ref_unchecked::<1, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_2_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&T, &Vector<2, T, VecPacked>) {
        self.get_n_n_ref_unchecked::<1, 2>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_3_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&T, &Vector<3, T, VecPacked>) {
        self.get_n_n_ref_unchecked::<1, 3>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_1_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&Vector<2, T, VecPacked>, &T) {
        self.get_n_n_ref_unchecked::<2, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_2_ref_unchecked(
        &self,
        indicies: [usize; 2],
    ) -> (&Vector<2, T, VecPacked>, &Vector<2, T, VecPacked>) {
        self.get_n_n_ref_unchecked::<2, 2>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_ref_unchecked(&self, indicies: [usize; 3]) -> (&T, &T, &T) {
        self.get_n_n_n_ref_unchecked::<1, 1, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_1_2_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&T, &T, &Vector<2, T, VecPacked>) {
        self.get_n_n_n_ref_unchecked::<1, 1, 2>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&T, &Vector<2, T, VecPacked>, &T) {
        self.get_n_n_n_ref_unchecked::<1, 2, 1>(indicies)
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_ref_unchecked(
        &self,
        indicies: [usize; 3],
    ) -> (&Vector<2, T, VecPacked>, &T, &T) {
        self.get_n_n_n_ref_unchecked::<2, 1, 1>(indicies)
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_ref_unchecked(&self, indicies: [usize; 4]) -> (&T, &T, &T, &T) {
        self.get_n_n_n_n_ref_unchecked::<1, 1, 1, 1>(indicies)
    }
}
