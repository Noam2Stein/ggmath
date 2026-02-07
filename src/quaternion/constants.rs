use crate::{
    Alignment, Quaternion, Scalar, Vector,
    constants::{Nan, One, Zero},
};

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Zero,
{
    /// All `0.0`.
    pub const ZERO: Self = Self::from_vec(Vector::ZERO);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Zero + One,
{
    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Nan,
{
    /// All NaNs (Not a Number).
    pub const NAN: Self = Self::from_vec(Vector::NAN);
}
