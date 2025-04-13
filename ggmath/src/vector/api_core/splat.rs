use super::*;

/// Creates a `Vec2` where all components are given the same value.
pub fn splat2<T: Scalar>(value: T) -> Vec2<T> {
    Vector::splat(value)
}
/// Creates a `Vec3` where all components are given the same value.
pub fn splat3<T: Scalar>(value: T) -> Vec3<T> {
    Vector::splat(value)
}
/// Creates a `Vec4` where all components are given the same value.
pub fn splat4<T: Scalar>(value: T) -> Vec4<T> {
    Vector::splat(value)
}

/// Creates a `Vec2P` where all components are given the same value.
pub fn splat2p<T: Scalar>(value: T) -> Vec2P<T> {
    Vector::splat(value)
}
/// Creates a `Vec3P` where all components are given the same value.
pub fn splat3p<T: Scalar>(value: T) -> Vec3P<T> {
    Vector::splat(value)
}
/// Creates a `Vec4P` where all components are given the same value.
pub fn splat4p<T: Scalar>(value: T) -> Vec4P<T> {
    Vector::splat(value)
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// Creates a vector where all components are given the same value.
    pub fn splat(value: T) -> Self {
        T::vector_splat(value)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_splat:

    #[inline(always)]
    fn vector_splat<const N: usize, A: VecAlignment>(value: Self) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array([value; N])
    }
}
