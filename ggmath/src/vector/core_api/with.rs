use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn with_n<const N_VALUE: usize>(
        self,
        index: usize,
        value: VectorOrScalar<N_VALUE, T, impl VecAlignment>,
    ) -> Option<Self>
    where
        ScalarCount<N_VALUE>: VecLenOr1,
    {
        match resolve_vector_or_scalar_length(value) {
            ResolvedVectorOrScalar::Scalar(value) => self.with(index, value),
            ResolvedVectorOrScalar::Vec2(value) => self.with_2(index, value),
            ResolvedVectorOrScalar::Vec3(value) => self.with_3(index, value),
            ResolvedVectorOrScalar::Vec4(value) => self.with_4(index, value),
        }
    }

    #[inline(always)]
    pub unsafe fn with_n_unchecked<const N_VALUE: usize, AValue: VecAlignment>(
        self,
        index: usize,
        value: VectorOrScalar<N_VALUE, T, AValue>,
    ) -> Self
    where
        ScalarCount<N_VALUE>: VecLenOr1,
    {
        match resolve_vector_or_scalar_length(value) {
            ResolvedVectorOrScalar::Scalar(value) => self.with_unchecked(index, value),
            ResolvedVectorOrScalar::Vec2(value) => self.with_2_unchecked(index, value),
            ResolvedVectorOrScalar::Vec3(value) => self.with_3_unchecked(index, value),
            ResolvedVectorOrScalar::Vec4(value) => self.with_4_unchecked(index, value),
        }
    }

    #[inline(always)]
    pub fn with(self, index: usize, value: T) -> Option<Self> {
        T::vector_with(self, index, value)
    }
    #[inline(always)]
    pub fn with_2(self, index: usize, value: Vector<2, T, impl VecAlignment>) -> Option<Self> {
        self.with_1_1([index, index + 1], value)
    }
    #[inline(always)]
    pub fn with_3(self, index: usize, value: Vector<3, T, impl VecAlignment>) -> Option<Self> {
        self.with_1_1_1([index, index + 1, index + 2], value)
    }
    #[inline(always)]
    pub fn with_4(self, index: usize, value: Vector<4, T, impl VecAlignment>) -> Option<Self> {
        self.with_1_1_1_1([index, index + 1, index + 2, index + 3], value)
    }

    #[inline(always)]
    pub fn with_1_1(
        self,
        indicies: [usize; 2],
        value: Vector<2, T, impl VecAlignment>,
    ) -> Option<Self> {
        T::vector_with_1_1(self, indicies, value)
    }
    #[inline(always)]
    pub fn with_1_2(
        self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1([indicies[0], indicies[1], indicies[1] + 1], value)
    }
    #[inline(always)]
    pub fn with_1_3(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[1], indicies[1] + 1, indicies[1] + 2],
            value,
        )
    }
    #[inline(always)]
    pub fn with_2_1(
        self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1([indicies[0], indicies[0] + 1, indicies[1]], value)
    }
    #[inline(always)]
    pub fn with_2_2(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[0] + 1, indicies[1], indicies[1] + 1],
            value,
        )
    }
    #[inline(always)]
    pub fn with_3_1(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[0] + 1, indicies[0] + 2, indicies[1]],
            value,
        )
    }

    #[inline(always)]
    pub fn with_1_1_1(
        self,
        indicies: [usize; 3],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Option<Self> {
        T::vector_with_1_1_1(self, indicies, value)
    }
    #[inline(always)]
    pub fn with_1_1_2(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[1], indicies[2], indicies[2] + 1],
            value,
        )
    }
    #[inline(always)]
    pub fn with_1_2_1(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[1], indicies[1] + 1, indicies[2]],
            value,
        )
    }
    #[inline(always)]
    pub fn with_2_1_1(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        self.with_1_1_1_1(
            [indicies[0], indicies[0] + 1, indicies[1], indicies[2]],
            value,
        )
    }

    #[inline(always)]
    pub fn with_1_1_1_1(
        self,
        indicies: [usize; 4],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Option<Self> {
        T::vector_with_1_1_1_1(self, indicies, value)
    }

    #[inline(always)]
    pub unsafe fn with_unchecked(self, index: usize, value: T) -> Self {
        T::vector_with_unchecked(self, index, value)
    }
    #[inline(always)]
    pub unsafe fn with_2_unchecked(
        self,
        index: usize,
        value: Vector<2, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_unchecked([index, index + 1], value)
    }
    #[inline(always)]
    pub unsafe fn with_3_unchecked(
        self,
        index: usize,
        value: Vector<3, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_unchecked([index, index + 1, index + 2], value)
    }
    #[inline(always)]
    pub unsafe fn with_4_unchecked(
        self,
        index: usize,
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked([index, index + 1, index + 2, index + 3], value)
    }

    #[inline(always)]
    pub unsafe fn with_1_1_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<2, T, impl VecAlignment>,
    ) -> Self {
        T::vector_with_1_1_unchecked(self, indicies, value)
    }
    #[inline(always)]
    pub unsafe fn with_1_2_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1], value)
    }
    #[inline(always)]
    pub unsafe fn with_1_3_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[1], indicies[1] + 1, indicies[1] + 2],
            value,
        )
    }
    #[inline(always)]
    pub unsafe fn with_2_1_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1]], value)
    }
    #[inline(always)]
    pub unsafe fn with_2_2_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[0] + 1, indicies[1], indicies[1] + 1],
            value,
        )
    }
    #[inline(always)]
    pub unsafe fn with_3_1_unchecked(
        self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[0] + 1, indicies[0] + 2, indicies[1]],
            value,
        )
    }

    #[inline(always)]
    pub unsafe fn with_1_1_1_unchecked(
        self,
        indicies: [usize; 3],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Self {
        T::vector_with_1_1_1_unchecked(self, indicies, value)
    }
    #[inline(always)]
    pub unsafe fn with_1_1_2_unchecked(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[1], indicies[2], indicies[2] + 1],
            value,
        )
    }
    #[inline(always)]
    pub unsafe fn with_1_2_1_unchecked(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[1], indicies[1] + 1, indicies[2]],
            value,
        )
    }
    #[inline(always)]
    pub unsafe fn with_2_1_1_unchecked(
        self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        self.with_1_1_1_1_unchecked(
            [indicies[0], indicies[0] + 1, indicies[1], indicies[2]],
            value,
        )
    }

    #[inline(always)]
    pub unsafe fn with_1_1_1_1_unchecked(
        self,
        indicies: [usize; 4],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Self {
        T::vector_with_1_1_1_1_unchecked(self, indicies, value)
    }
}
