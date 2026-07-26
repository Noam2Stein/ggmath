/// A trait for types with a `0` value.
///
/// This is used for generic functions that require `0`, like
/// [`Matrix::from_diagonal`].
///
/// [`Matrix::from_diagonal`]: crate::Matrix::from_diagonal
pub trait Zero {
    /// `0`.
    const ZERO: Self;
}

/// A trait for types with a `1` value.
///
/// This is used for generic functions that require `1`, like
/// [`Affine::from_translation`].
///
/// [`Affine::from_translation`]: crate::Affine::from_translation
pub trait One {
    /// `1`.
    const ONE: Self;
}

/// A trait for types with a `-1` value.
///
/// Currently this is used by vector constants, like [`Vector::NEG_ONE`].
///
/// [`Vector::NEG_ONE`]: crate::Vector::NEG_ONE
pub trait NegOne {
    /// `-1`.
    const NEG_ONE: Self;
}

macro_rules! float_impl {
    ($T:ident) => {
        impl Zero for $T {
            const ZERO: Self = 0.0;
        }

        impl One for $T {
            const ONE: Self = 1.0;
        }

        impl NegOne for $T {
            const NEG_ONE: Self = -1.0;
        }
    };
}
float_impl!(f32);
float_impl!(f64);

macro_rules! integer_impl {
    ($T:ident) => {
        impl Zero for $T {
            const ZERO: Self = 0;
        }

        impl One for $T {
            const ONE: Self = 1;
        }
    };
}
integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
integer_impl!(i128);
integer_impl!(isize);
integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(u64);
integer_impl!(u128);
integer_impl!(usize);

macro_rules! signed_impl {
    ($T:ident) => {
        impl NegOne for $T {
            const NEG_ONE: Self = -1;
        }
    };
}
signed_impl!(i8);
signed_impl!(i16);
signed_impl!(i32);
signed_impl!(i64);
signed_impl!(i128);
signed_impl!(isize);
