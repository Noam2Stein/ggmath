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
        T::vector_get_unchecked(self, index)
    }
    #[inline(always)]
    pub unsafe fn get_2_unchecked(self, index: usize) -> Vector<2, T, A> {
        self.get_1_1_unchecked([index, index + 1])
    }
    #[inline(always)]
    pub unsafe fn get_3_unchecked(self, index: usize) -> Vector<3, T, A> {
        self.get_1_1_1_unchecked([index, index + 1, index + 2])
    }
    #[inline(always)]
    pub unsafe fn get_4_unchecked(self, index: usize) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([index, index + 1, index + 2, index + 3])
    }

    #[inline(always)]
    pub unsafe fn get_1_1_unchecked(self, indicies: [usize; 2]) -> Vector<2, T, A> {
        T::vector_get_1_1_unchecked(self, indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_2_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {
        self.get_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1])
    }
    #[inline(always)]
    pub unsafe fn get_1_3_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1, indicies[1] + 2])
    }
    #[inline(always)]
    pub unsafe fn get_2_1_unchecked(self, indicies: [usize; 2]) -> Vector<3, T, A> {
        self.get_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1]])
    }
    #[inline(always)]
    pub unsafe fn get_2_2_unchecked(self, indicies: [usize; 2]) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1], indicies[1] + 1])
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<3, T, A> {
        T::vector_get_1_1_1_unchecked(self, indicies)
    }
    #[inline(always)]
    pub unsafe fn get_1_1_2_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[2], indicies[2] + 1])
    }
    #[inline(always)]
    pub unsafe fn get_1_2_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([indicies[0], indicies[1], indicies[1] + 1, indicies[2]])
    }
    #[inline(always)]
    pub unsafe fn get_2_1_1_unchecked(self, indicies: [usize; 3]) -> Vector<4, T, A> {
        self.get_1_1_1_1_unchecked([indicies[0], indicies[0] + 1, indicies[1], indicies[2]])
    }

    #[inline(always)]
    pub unsafe fn get_1_1_1_1_unchecked(self, indicies: [usize; 4]) -> Vector<4, T, A> {
        T::vector_get_1_1_1_1_unchecked(self, indicies)
    }
}
