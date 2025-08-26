use std::hash::{Hash, Hasher};

use super::*;

impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::vec_hash(self, state);
    }
}
