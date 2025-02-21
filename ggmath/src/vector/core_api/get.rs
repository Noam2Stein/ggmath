use std::mem::transmute_copy;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn get_n<const N_OUTPUT: usize>(self, index: usize) -> Option<VectorOrT<N_OUTPUT, T, A>>
    where
        ScalarCount<N_OUTPUT>: VecLenOr1,
    {
        unsafe {
            match N_OUTPUT {
                1 => transmute_copy(&self.get(index)),
                2 => transmute_copy(&self.get_2(index)),
                3 => transmute_copy(&self.get_3(index)),
                4 => transmute_copy(&self.get_4(index)),
                _ => panic!("invalid VecLen"),
            }
        }
    }

    #[inline(always)]
    pub fn get(self, index: usize) -> Option<T> {
        T::vector_get(self, index)
    }
    #[inline(always)]
    pub fn get_2(self, index: usize) -> Option<Vector<2, T, A>> {
        self.get_1_1([index, index + 1])
    }
    #[inline(always)]
    pub fn get_3(self, index: usize) -> Option<Vector<3, T, A>> {
        self.get_1_1_1([index, index + 1, index + 2])
    }
    #[inline(always)]
    pub fn get_4(self, index: usize) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([index, index + 1, index + 2, index + 3])
    }

    #[inline(always)]
    pub fn get_1_1(self, indicies: [usize; 2]) -> Option<Vector<2, T, A>> {
        T::vector_get_1_1(self, indicies)
    }
    #[inline(always)]
    pub fn get_1_2(self, indicies: [usize; 2]) -> Option<Vector<3, T, A>> {
        self.get_1_1_1([indicies[0], indicies[1], indicies[1] + 1])
    }
    #[inline(always)]
    pub fn get_1_3(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[1], indicies[1] + 1, indicies[1] + 2])
    }
    #[inline(always)]
    pub fn get_2_1(self, indicies: [usize; 2]) -> Option<Vector<3, T, A>> {
        self.get_1_1_1([indicies[0], indicies[0] + 1, indicies[1]])
    }
    #[inline(always)]
    pub fn get_2_2(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[0] + 1, indicies[1], indicies[1] + 1])
    }
    #[inline(always)]
    pub fn get_3_1(self, indicies: [usize; 2]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[0] + 1, indicies[0] + 2, indicies[1]])
    }

    #[inline(always)]
    pub fn get_1_1_1(self, indicies: [usize; 3]) -> Option<Vector<3, T, A>> {
        T::vector_get_1_1_1(self, indicies)
    }
    #[inline(always)]
    pub fn get_1_1_2(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[1], indicies[2], indicies[2] + 1])
    }
    #[inline(always)]
    pub fn get_1_2_1(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[1], indicies[1] + 1, indicies[2]])
    }
    #[inline(always)]
    pub fn get_2_1_1(self, indicies: [usize; 3]) -> Option<Vector<4, T, A>> {
        self.get_1_1_1_1([indicies[0], indicies[0] + 1, indicies[1], indicies[2]])
    }

    #[inline(always)]
    pub fn get_1_1_1_1(self, indicies: [usize; 4]) -> Option<Vector<4, T, A>> {
        T::vector_get_1_1_1_1(self, indicies)
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(self, index: usize) -> T {
        unsafe { T::vector_get_unchecked(self, index) }
    }
    #[inline(always)]
    pub unsafe fn get_2_unchecked(self, index: usize) -> Vector<2, T, A> {
        unsafe { self.get_1_1_unchecked([index, index + 1]) }
    }
    #[inline(always)]
    pub unsafe fn get_3_unchecked(self, index: usize) -> Vector<3, T, A> {
        unsafe { self.get_1_1_1_unchecked([index, index + 1, index + 2]) }
    }
    #[inline(always)]
    pub unsafe fn get_4_unchecked(self, index: usize) -> Vector<4, T, A> {
        unsafe { self.get_1_1_1_1_unchecked([index, index + 1, index + 2, index + 3]) }
    }

    #[inline(always)]
    pub unsafe fn get_1_1_unchecked(self, indicies: [usize; 2]) -> Vector<2, T, A> {
        unsafe { T::vector_get_1_1_unchecked(self, indicies) }
    }
    #[inline(always)]
    pub unsafe fn get_1_2_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {
        unsafe { self.get_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1]) }
    }
    #[inline(always)]
    pub unsafe fn get_1_3_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {
        unsafe {
            self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1, indicies[1] + 2])
        }
    }
    #[inline(always)]
    pub unsafe fn get_2_1_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {
        unsafe { self.get_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1]]) }
    }
    #[inline(always)]
    pub unsafe fn get_2_2_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {
        unsafe {
            self.get_1_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1], indicies[1] + 1])
        }
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<3, T, A> {
        unsafe { T::vector_get_1_1_1_unchecked(self, indicies) }
    }
    #[inline(always)]
    pub unsafe fn get_1_1_2_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        unsafe {
            self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[2], indicies[2] + 1])
        }
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        unsafe {
            self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1, indicies[2]])
        }
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        unsafe {
            self.get_1_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1], indicies[2]])
        }
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_unchecked(self, indicies: [usize; 4]) -> Vector<4, T, A> {
        unsafe { T::vector_get_1_1_1_1_unchecked(self, indicies) }
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_get:

    splat_attribs::splat_attribs! {
        #[inline(always)]:

        fn vector_get<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            index: usize,
        ) -> Option<Self>
        where
            ScalarCount<N>: VecLen,
        {
            vec.get_ref(index).map(|output| *output)
        }

        fn vector_get_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 2],
        ) -> Option<Vector<2, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            vec.get_1_1_ref(indicies)
                .map(|(output0, output1)| Vector::<2, Self, A>::from_array([*output0, *output1]))
        }

        fn vector_get_1_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 3],
        ) -> Option<Vector<3, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            vec.get_1_1_1_ref(indicies)
                .map(|(output0, output1, output2)| {
                    Vector::<3, Self, A>::from_array([*output0, *output1, *output2])
                })
        }

        fn vector_get_1_1_1_1<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 4],
        ) -> Option<Vector<4, Self, A>>
        where
            ScalarCount<N>: VecLen,
        {
            vec.get_1_1_1_1_ref(indicies)
                .map(|(output0, output1, output2, output3)| {
                    Vector::<4, Self, A>::from_array([*output0, *output1, *output2, *output3])
                })
        }

        unsafe fn vector_get_unchecked<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            index: usize,
        ) -> Self
        where
            ScalarCount<N>: VecLen,
        {
            unsafe {
                *vec.as_array().get_unchecked(index)
            }
        }

        unsafe fn vector_get_1_1_unchecked<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 2],
        ) -> Vector<2, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            Vector::<2, Self, A>::from_array(unsafe{[
                vec.get_unchecked(indicies[0]),
                vec.get_unchecked(indicies[1]),
            ]})
        }

        unsafe fn vector_get_1_1_1_unchecked<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 3],
        ) -> Vector<3, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            Vector::<3, Self, A>::from_array(unsafe {
                [
                vec.get_unchecked(indicies[0]),
                vec.get_unchecked(indicies[1]),
                vec.get_unchecked(indicies[2]),
            ]
            })
        }

        unsafe fn vector_get_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
            indicies: [usize; 4],
        ) -> Vector<4, Self, A>
        where
            ScalarCount<N>: VecLen,
        {
            Vector::<4, Self, A>::from_array(unsafe { [
                vec.get_unchecked(indicies[0]),
                vec.get_unchecked(indicies[1]),
                vec.get_unchecked(indicies[2]),
                vec.get_unchecked(indicies[3]),
            ]})
        }
    }
}
