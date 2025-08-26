use super::*;

/// Creates a `Vec2` where all components are given the same value.
#[inline(always)]
pub const fn splat2<T: Scalar>(value: T) -> Vec2<T> {
    Vector::splat(value)
}

/// Creates a `Vec2P` where all components are given the same value.
#[inline(always)]
pub const fn splat2p<T: Scalar>(value: T) -> Vec2P<T> {
    Vector::splat(value)
}

/// Creates a `Vector<2, _, _>` where all components are given the same value.
#[inline(always)]
pub const fn splat2g<T: Scalar, A: VecAlignment>(value: T) -> Vector<2, T, A> {
    Vector::splat(value)
}

/// Creates a `Vec3` where all components are given the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vec3<T> {
    Vector::splat(value)
}

/// Creates a `Vec3P` where all components are given the same value.
#[inline(always)]
pub const fn splat3p<T: Scalar>(value: T) -> Vec3P<T> {
    Vector::splat(value)
}

/// Creates a `Vector<3, _, _>` where all components are given the same value.
#[inline(always)]
pub const fn splat3g<T: Scalar, A: VecAlignment>(value: T) -> Vector<3, T, A> {
    Vector::splat(value)
}

/// Creates a `Vec4` where all components are given the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vec4<T> {
    Vector::splat(value)
}

/// Creates a `Vec4P` where all components are given the same value.
#[inline(always)]
pub const fn splat4p<T: Scalar>(value: T) -> Vec4P<T> {
    Vector::splat(value)
}

/// Creates a `Vector<4, _, _>` where all components are given the same value.
#[inline(always)]
pub const fn splat4g<T: Scalar, A: VecAlignment>(value: T) -> Vector<4, T, A> {
    Vector::splat(value)
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Creates a vector where all components are given the same value.
    #[inline(always)]
    pub const fn splat(value: T) -> Self {
        Self::from_array([value; N])
    }
}
