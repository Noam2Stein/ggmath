use crate::{
    Alignment, Length, Scalar, SupportedLength, Vector,
    constants::{False, Infinity, Max, Min, Nan, NegInfinity, NegOne, One, True, Zero},
};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// `0`.
    pub const ZERO: Self = Self::splat(T::ZERO);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + One,
{
    /// All `1`.
    pub const ONE: Self = Self::splat(T::ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegOne,
{
    /// All `-1`.
    pub const NEG_ONE: Self = Self::splat(T::NEG_ONE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Min,
{
    /// The minimum value representable by `T` for all elements.
    pub const MIN: Self = Self::splat(T::MIN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Max,
{
    /// The maximum value representable by `T` for all elements.
    pub const MAX: Self = Self::splat(T::MAX);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// All elements set to Not a Number (NaN).
    pub const NAN: Self = Self::splat(T::NAN);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Infinity,
{
    /// All elements set to Infinity (∞).
    pub const INFINITY: Self = Self::splat(T::INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + NegInfinity,
{
    /// All elements set to Negative Infinity (-∞).
    pub const NEG_INFINITY: Self = Self::splat(T::NEG_INFINITY);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + True,
{
    /// All `true`.
    pub const TRUE: Self = Self::splat(T::TRUE);
}

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + False,
{
    /// All `false`.
    pub const FALSE: Self = Self::splat(T::FALSE);
}

////////////////////////////////////////////////////////////////////////////////
// Axes
////////////////////////////////////////////////////////////////////////////////

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO);

    /// `(0, 1)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

    /// `(0, 1, 0)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

    /// `(0, 0, 1)`.
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar + Zero + One,
{
    /// `(1, 0, 0, 0)`.
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// `(0, 1, 0, 0)`.
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// `(0, 0, 1, 0)`.
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// `(0, 0, 0, 1)`.
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO);

    /// `(0, -1)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE);
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO);

    /// `(0, -1, 0)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO);

    /// `(0, 0, -1)`.
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE);
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar + Zero + NegOne,
{
    /// `(-1, 0, 0, 0)`.
    pub const NEG_X: Self = Self::new(T::NEG_ONE, T::ZERO, T::ZERO, T::ZERO);

    /// `(0, -1, 0, 0)`.
    pub const NEG_Y: Self = Self::new(T::ZERO, T::NEG_ONE, T::ZERO, T::ZERO);

    /// `(0, 0, -1, 0)`.
    pub const NEG_Z: Self = Self::new(T::ZERO, T::ZERO, T::NEG_ONE, T::ZERO);

    /// `(0, 0, 0, -1)`.
    pub const NEG_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::NEG_ONE);
}
