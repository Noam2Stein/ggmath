use super::*;

#[inline(always)]
/// Creates a `Vec2` where all components are given the same value.
pub fn splat2<T: Scalar>(value: T) -> Vec2<T> {
    Vector::splat(value)
}

#[inline(always)]
/// Creates a `Vec3` where all components are given the same value.
pub const fn splat3<T: Scalar>(value: T) -> Vec3<T> {
    Vector::splat(value)
}

#[inline(always)]
/// Creates a `Vec4` where all components are given the same value.
pub const fn splat4<T: Scalar>(value: T) -> Vec4<T> {
    Vector::splat(value)
}

#[inline(always)]
/// Creates a `Vec2P` where all components are given the same value.
pub const fn splat2p<T: Scalar>(value: T) -> Vec2P<T> {
    Vector::splat(value)
}

#[inline(always)]
/// Creates a `Vec3P` where all components are given the same value.
pub const fn splat3p<T: Scalar>(value: T) -> Vec3P<T> {
    Vector::splat(value)
}

#[inline(always)]
/// Creates a `Vec4P` where all components are given the same value.
pub const fn splat4p<T: Scalar>(value: T) -> Vec4P<T> {
    Vector::splat(value)
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    /// Creates a vector where all components are given the same value.
    pub const fn splat(value: T) -> Self {
        Self::from_array([value; N])
    }
}
