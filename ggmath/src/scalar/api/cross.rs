use super::*;

pub trait ScalarCross: ScalarMul<Output = Self> + ScalarSub<Output = Self> {
    #[inline(always)]
    fn vector_cross<A: VecAlignment>(
        vec: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A> {
        (vec.zxy() * other - vec * other.zxy()).zxy()
    }
}

impl<T: ScalarCross, A: VecAlignment> Vector<3, T, A> {
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        T::vector_cross(self, other)
    }
}
