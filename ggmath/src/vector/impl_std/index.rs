use super::*;

use std::ops::{Index, IndexMut};

impl<const N: usize, T: Scalar, A: VecAlignment> Index<usize> for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array().index(index)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> IndexMut<usize> for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}
