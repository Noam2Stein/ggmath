use splat_attribs::splat_attribs;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    splat_attribs! {
        #[inline(always)]:

        pub fn with_n<const N_VALUE: usize>(
            self,
            index: usize,
            value: VectorOrT<N_VALUE, T, impl VecAlignment>,
        ) -> Option<Self>
        where
            ScalarCount<N_VALUE>: VecLenOr1,
        {
            match resolve_vector_or_t_length(value) {
                ResolvedVectorOrT::T(value) => self.with(index, value),
                ResolvedVectorOrT::Vec2(value) => self.with_2(index, value),
                ResolvedVectorOrT::Vec3(value) => self.with_3(index, value),
                ResolvedVectorOrT::Vec4(value) => self.with_4(index, value),
            }
        }

        pub unsafe fn with_n_unchecked<const N_VALUE: usize, AValue: VecAlignment>(
            self,
            index: usize,
            value: VectorOrT<N_VALUE, T, AValue>,
        ) -> Self
        where
            ScalarCount<N_VALUE>: VecLenOr1,
        {
            match resolve_vector_or_t_length(value) {
                ResolvedVectorOrT::T(value) => self.with_unchecked(index, value),
                ResolvedVectorOrT::Vec2(value) => self.with_2_unchecked(index, value),
                ResolvedVectorOrT::Vec3(value) => self.with_3_unchecked(index, value),
                ResolvedVectorOrT::Vec4(value) => self.with_4_unchecked(index, value),
            }
        }

        pub fn with(self, index: usize, value: T) -> Option<Self> {
            T::vector_with(self, index, value)
        }
        pub fn with_2(self, index: usize, value: Vector<2, T, impl VecAlignment>) -> Option<Self> {
            self.with_1_1([index, index + 1], value)
        }
        pub fn with_3(self, index: usize, value: Vector<3, T, impl VecAlignment>) -> Option<Self> {
            self.with_1_1_1([index, index + 1, index + 2], value)
        }
        pub fn with_4(self, index: usize, value: Vector<4, T, impl VecAlignment>) -> Option<Self> {
            self.with_1_1_1_1([index, index + 1, index + 2, index + 3], value)
        }

        pub fn with_1_1(
            self,
            indicies: [usize; 2],
            value: Vector<2, T, impl VecAlignment>,
        ) -> Option<Self> {
            T::vector_with_1_1(self, indicies, value)
        }
        pub fn with_1_2(
            self,
            indicies: [usize; 2],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Option<Self> {
            self.with_1_1_1([indicies[0], indicies[1], indicies[1] + 1], value)
        }
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
        pub fn with_2_1(
            self,
            indicies: [usize; 2],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Option<Self> {
            self.with_1_1_1([indicies[0], indicies[0] + 1, indicies[1]], value)
        }
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

        pub fn with_1_1_1(
            self,
            indicies: [usize; 3],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Option<Self> {
            T::vector_with_1_1_1(self, indicies, value)
        }
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

        pub fn with_1_1_1_1(
            self,
            indicies: [usize; 4],
            value: Vector<4, T, impl VecAlignment>,
        ) -> Option<Self> {
            T::vector_with_1_1_1_1(self, indicies, value)
        }

        pub unsafe fn with_unchecked(self, index: usize, value: T) -> Self {
            T::vector_with_unchecked(self, index, value)
        }
        pub unsafe fn with_2_unchecked(
            self,
            index: usize,
            value: Vector<2, T, impl VecAlignment>,
        ) -> Self {
            self.with_1_1_unchecked([index, index + 1], value)
        }
        pub unsafe fn with_3_unchecked(
            self,
            index: usize,
            value: Vector<3, T, impl VecAlignment>,
        ) -> Self {
            self.with_1_1_1_unchecked([index, index + 1, index + 2], value)
        }
        pub unsafe fn with_4_unchecked(
            self,
            index: usize,
            value: Vector<4, T, impl VecAlignment>,
        ) -> Self {
            self.with_1_1_1_1_unchecked([index, index + 1, index + 2, index + 3], value)
        }

        pub unsafe fn with_1_1_unchecked(
            self,
            indicies: [usize; 2],
            value: Vector<2, T, impl VecAlignment>,
        ) -> Self {
            T::vector_with_1_1_unchecked(self, indicies, value)
        }
        pub unsafe fn with_1_2_unchecked(
            self,
            indicies: [usize; 2],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Self {
            self.with_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1], value)
        }
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
        pub unsafe fn with_2_1_unchecked(
            self,
            indicies: [usize; 2],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Self {
            self.with_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1]], value)
        }
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

        pub unsafe fn with_1_1_1_unchecked(
            self,
            indicies: [usize; 3],
            value: Vector<3, T, impl VecAlignment>,
        ) -> Self {
            T::vector_with_1_1_1_unchecked(self, indicies, value)
        }
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

        pub unsafe fn with_1_1_1_1_unchecked(
            self,
            indicies: [usize; 4],
            value: Vector<4, T, impl VecAlignment>,
        ) -> Self {
            T::vector_with_1_1_1_1_unchecked(self, indicies, value)
        }
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_with:

    splat_attribs::splat_attribs! {
        #[inline(always)]:

        fn vector_with<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            index: usize,
            value: Self,
        ) -> Option<Vector<N, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            if index >= N {
                None
            } else {
                Some(unsafe { vec.with_unchecked(index, value) })
            }
        }

        fn vector_with_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 2],
            value: Vector<2, Self, impl VecAlignment>,
        ) -> Option<Vector<N, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            if indicies.into_iter().any(|index| index >= N) {
                None
            } else if indicies[0] == indicies[1] {
                None
            } else {
                Some(unsafe { vec.with_1_1_unchecked(indicies, value) })
            }
        }

        fn vector_with_1_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 3],
            value: Vector<3, Self, impl VecAlignment>,
        ) -> Option<Vector<N, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            if indicies.into_iter().any(|index| index >= N) {
                None
            } else if indicies[0] == indicies[1] {
                None
            } else if indicies[0] == indicies[2] {
                None
            } else if indicies[1] == indicies[2] {
                None
            } else {
                Some(unsafe { vec.with_1_1_1_unchecked(indicies, value) })
            }
        }

        fn vector_with_1_1_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 4],
            value: Vector<4, Self, impl VecAlignment>,
        ) -> Option<Vector<N, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            if indicies.into_iter().any(|index| index >= N) {
                None
            } else if indicies[0] == indicies[1] {
                None
            } else if indicies[0] == indicies[2] {
                None
            } else if indicies[0] == indicies[3] {
                None
            } else if indicies[1] == indicies[2] {
                None
            } else if indicies[1] == indicies[3] {
                None
            } else if indicies[2] == indicies[3] {
                None
            } else {
                Some(unsafe { vec.with_1_1_1_1_unchecked(indicies, value) })
            }
        }

        unsafe fn vector_with_unchecked<const N: usize, A: VecAlignment>(
            mut vec: Vector<N, Self, A>,
            index: usize,
            value: Self,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            *vec.get_mut_unchecked(index) = value;
            vec
        }

        unsafe fn vector_with_1_1_unchecked<const N: usize, A: VecAlignment>(
            mut vec: Vector<N, Self, A>,
            indicies: [usize; 2],
            value: Vector<2, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
            *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
            vec
        }

        unsafe fn vector_with_1_1_1_unchecked<const N: usize, A: VecAlignment>(
            mut vec: Vector<N, Self, A>,
            indicies: [usize; 3],
            value: Vector<3, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
            *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
            *vec.get_mut_unchecked(indicies[2]) = value.get_unchecked(2);
            vec
        }

        unsafe fn vector_with_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
            mut vec: Vector<N, Self, A>,
            indicies: [usize; 4],
            value: Vector<4, Self, impl VecAlignment>,
        ) -> Vector<N, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
            *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
            *vec.get_mut_unchecked(indicies[2]) = value.get_unchecked(2);
            *vec.get_mut_unchecked(indicies[3]) = value.get_unchecked(3);
            vec
        }
    }
}
