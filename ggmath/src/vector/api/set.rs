use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn set_n<const NValue: usize, AValue: VecAlignment>(
        &mut self,
        index: usize,
        value: VectorOrScalar<NValue, T, AValue>,
    ) -> Result<(), ()>
    where
        ScalarCount<NValue>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub fn set_n_n<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 2],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
        ),
    ) -> Result<(), ()>
    where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub fn set_n_n_n<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
        const NValue2: usize,
        AValue2: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 3],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
            VectorOrScalar<NValue2, T, AValue2>,
        ),
    ) -> Result<(), ()>
    where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
        ScalarCount<NValue2>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub fn set_n_n_n_n<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
        const NValue2: usize,
        AValue2: VecAlignment,
        const NValue3: usize,
        AValue3: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 4],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
            VectorOrScalar<NValue2, T, AValue2>,
            VectorOrScalar<NValue3, T, AValue3>,
        ),
    ) -> Result<(), ()>
    where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
        ScalarCount<NValue2>: VecLenOr1,
        ScalarCount<NValue3>: VecLenOr1,
    {
    }

    #[inline(always)]
    pub unsafe fn set_n_unchecked<const NValue: usize, AValue: VecAlignment>(
        &mut self,
        index: usize,
        value: VectorOrScalar<NValue, T, AValue>,
    ) where
        ScalarCount<NValue>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub unsafe fn set_n_n_unchecked<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 2],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
        ),
    ) where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub unsafe fn set_n_n_n_unchecked<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
        const NValue2: usize,
        AValue2: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 3],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
            VectorOrScalar<NValue2, T, AValue2>,
        ),
    ) where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
        ScalarCount<NValue2>: VecLenOr1,
    {
    }
    #[inline(always)]
    pub unsafe fn set_n_n_n_n_unchecked<
        const NValue0: usize,
        AValue0: VecAlignment,
        const NValue1: usize,
        AValue1: VecAlignment,
        const NValue2: usize,
        AValue2: VecAlignment,
        const NValue3: usize,
        AValue3: VecAlignment,
    >(
        &mut self,
        indicies: [usize; 4],
        values: (
            VectorOrScalar<NValue0, T, AValue0>,
            VectorOrScalar<NValue1, T, AValue1>,
            VectorOrScalar<NValue2, T, AValue2>,
            VectorOrScalar<NValue3, T, AValue3>,
        ),
    ) where
        ScalarCount<NValue0>: VecLenOr1,
        ScalarCount<NValue1>: VecLenOr1,
        ScalarCount<NValue2>: VecLenOr1,
        ScalarCount<NValue3>: VecLenOr1,
    {
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
        self.set_n::<1, VecPacked>(index, value)
    }
    #[inline(always)]
    pub fn set_2(
        &mut self,
        index: usize,
        value: Vector<2, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        self.set_n::<2, _>(index, value)
    }
    #[inline(always)]
    pub fn set_3(
        &mut self,
        index: usize,
        value: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        self.set_n::<3, _>(index, value)
    }
    #[inline(always)]
    pub fn set_4(
        &mut self,
        index: usize,
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        self.set_n::<4, _>(index, value)
    }

    #[inline(always)]
    pub fn set_1_1(&mut self, indicies: [usize; 2], values: (T, T)) -> Result<(), ()> {
        self.set_n_n::<1, VecPacked, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub fn set_1_2(
        &mut self,
        indicies: [usize; 2],
        values: (T, Vector<2, T, impl VecAlignment>),
    ) -> Result<(), ()> {
        self.set_n_n::<1, VecPacked, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub fn set_1_3(
        &mut self,
        indicies: [usize; 2],
        values: (T, Vector<3, T, impl VecAlignment>),
    ) -> Result<(), ()> {
        self.set_n_n::<1, VecPacked, 3, _>(indicies, values)
    }
    #[inline(always)]
    pub fn set_2_1(
        &mut self,
        indicies: [usize; 2],
        values: (Vector<2, T, impl VecAlignment>, T),
    ) -> Result<(), ()> {
        self.set_n_n::<2, _, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub fn set_2_2(
        &mut self,
        indicies: [usize; 2],
        values: (
            Vector<2, T, impl VecAlignment>,
            Vector<2, T, impl VecAlignment>,
        ),
    ) -> Result<(), ()> {
        self.set_n_n::<2, _, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub fn set_3_1(
        &mut self,
        indicies: [usize; 2],
        values: (Vector<3, T, impl VecAlignment>, T),
    ) -> Result<(), ()> {
        self.set_n_n::<3, _, 1, VecPacked>(indicies, values)
    }

    #[inline(always)]
    pub fn set_1_1_1(&mut self, indicies: [usize; 3], values: (T, T, T)) -> Result<(), ()> {
        self.set_n_n_n::<1, VecPacked, 1, VecPacked, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub fn set_1_1_2(
        &mut self,
        indicies: [usize; 3],
        values: (T, T, Vector<2, T, impl VecAlignment>),
    ) -> Result<(), ()> {
        self.set_n_n_n::<1, VecPacked, 1, VecPacked, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub fn set_1_2_1(
        &mut self,
        indicies: [usize; 3],
        values: (T, Vector<2, T, impl VecAlignment>, T),
    ) -> Result<(), ()> {
        self.set_n_n_n::<1, VecPacked, 2, _, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub fn set_2_1_1(
        &mut self,
        indicies: [usize; 3],
        values: (Vector<2, T, impl VecAlignment>, T, T),
    ) -> Result<(), ()> {
        self.set_n_n_n::<2, _, 1, VecPacked, 1, VecPacked>(indicies, values)
    }

    #[inline(always)]
    pub fn set_1_1_1_1(&mut self, indicies: [usize; 4], values: (T, T, T, T)) -> Result<(), ()> {
        self.set_n_n_n_n::<1, VecPacked, 1, VecPacked, 1, VecPacked, 1, VecPacked>(indicies, values)
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        self.set_n_unchecked::<1, VecPacked>(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_2_unchecked(&mut self, index: usize, value: Vector<2, T, impl VecAlignment>) {
        self.set_n_unchecked::<2, _>(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_3_unchecked(&mut self, index: usize, value: Vector<3, T, impl VecAlignment>) {
        self.set_n_unchecked::<3, _>(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_4_unchecked(&mut self, index: usize, value: Vector<4, T, impl VecAlignment>) {
        self.set_n_unchecked::<4, _>(index, value)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_unchecked(&mut self, indicies: [usize; 2], values: (T, T)) {
        self.set_n_n_unchecked::<1, VecPacked, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_1_2_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: (T, Vector<2, T, impl VecAlignment>),
    ) {
        self.set_n_n_unchecked::<1, VecPacked, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_1_3_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: (T, Vector<3, T, impl VecAlignment>),
    ) {
        self.set_n_n_unchecked::<1, VecPacked, 3, _>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_2_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: (Vector<2, T, impl VecAlignment>, T),
    ) {
        self.set_n_n_unchecked::<2, _, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_2_2_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: (
            Vector<2, T, impl VecAlignment>,
            Vector<2, T, impl VecAlignment>,
        ),
    ) {
        self.set_n_n_unchecked::<2, _, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_3_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: (Vector<3, T, impl VecAlignment>, T),
    ) {
        self.set_n_n_unchecked::<3, _, 1, VecPacked>(indicies, values)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_1_unchecked(&mut self, indicies: [usize; 3], values: (T, T, T)) {
        self.set_n_n_n_unchecked::<1, VecPacked, 1, VecPacked, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_1_1_2_unchecked(
        &mut self,
        indicies: [usize; 3],
        values: (T, T, Vector<2, T, impl VecAlignment>),
    ) {
        self.set_n_n_n_unchecked::<1, VecPacked, 1, VecPacked, 2, _>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_1_2_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        values: (T, Vector<2, T, impl VecAlignment>, T),
    ) {
        self.set_n_n_n_unchecked::<1, VecPacked, 2, _, 1, VecPacked>(indicies, values)
    }
    #[inline(always)]
    pub unsafe fn set_2_1_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        values: (Vector<2, T, impl VecAlignment>, T, T),
    ) {
        self.set_n_n_n_unchecked::<2, _, 1, VecPacked, 1, VecPacked>(indicies, values)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_1_1_unchecked(&mut self, indicies: [usize; 4], values: (T, T, T, T)) {
        self.set_n_n_n_n_unchecked::<1, VecPacked, 1, VecPacked, 1, VecPacked, 1, VecPacked>(
            indicies, values,
        )
    }
}

#[inline(always)]
fn range_intersects(a_start: usize, a_len: usize, b_start: usize, b_len: usize) -> bool {
    a_start < b_start + b_len && b_start < a_start + a_len
}
