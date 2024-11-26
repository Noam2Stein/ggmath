use super::*;

impl<T: Scalar + Mul<Output = T> + Sub<Output = T>, A: VecAlignment> Vector<3, T, A> {
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        T::vector_cross(self, other)
    }
}
