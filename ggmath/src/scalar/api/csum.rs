use super::*;

pub trait ScalarCSum: ScalarAdd<Output = Self> {
    #[inline(always)]
    fn vector_csum<const N: usize>(vec: Vector<N, Self, impl VecAlignment>) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        match vec.resolve_length() {
            LengthResolvedVector::Vec2(vec) => vec.x() + vec.y(),
            LengthResolvedVector::Vec3(vec) => vec.x() + vec.y() + vec.z(),
            LengthResolvedVector::Vec4(vec) => vec.x() + vec.y() + vec.z() + vec.w(),
        }
    }
}

impl<const N: usize, T: ScalarCSum, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn csum(self) -> T {
        T::vector_csum(self)
    }
}
