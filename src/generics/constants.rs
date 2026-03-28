//! A module with traits for scalar constants like `ZERO`, `ONE` and `NAN`.
//!
//! Each constant has a trait which when implemented enables that constant for
//! vectors and other math types where appropriate.

use crate::Scalar;

/// A `ZERO` constant for scalar types.
pub trait Zero: Scalar {
    /// `0`.
    const ZERO: Self;
}

/// A `ONE` constant for scalar types.
pub trait One: Scalar {
    /// `1`.
    const ONE: Self;
}

/// A `NEG_ONE` constant for scalar types.
pub trait NegOne: Scalar {
    /// `-1`.
    const NEG_ONE: Self;
}

/// A `MIN` constant for scalar types.
pub trait Min: Scalar {
    /// The smallest value representable by `Self`.
    const MIN: Self;
}

/// A `MAX` constant for scalar types.
pub trait Max: Scalar {
    /// The largest value representable by `Self`.
    const MAX: Self;
}

/// A `NAN` constant for scalar types.
pub trait Nan: Scalar {
    /// Not a Number (NaN).
    const NAN: Self;
}

/// An `INFINITY` constant for scalar types.
pub trait Infinity: Scalar {
    /// Infinity (∞).
    const INFINITY: Self;
}

/// A `NEG_INFINITY` constant for scalar types.
pub trait NegInfinity: Scalar {
    /// Negative infinity (−∞).
    const NEG_INFINITY: Self;
}

/// A `TRUE` constant for scalar types.
pub trait True: Scalar {
    /// `true`.
    const TRUE: Self;
}

/// A `FALSE` constant for scalar types.
pub trait False: Scalar {
    /// `false`.
    const FALSE: Self;
}

macro_rules! impl_float {
    ($($T:ty),*$(,)?) => {
        $(
            impl Zero for $T {
                const ZERO: Self = 0.0;
            }

            impl One for $T {
                const ONE: Self = 1.0;
            }

            impl NegOne for $T {
                const NEG_ONE: Self = -1.0;
            }

            impl Min for $T {
                const MIN: Self = Self::MIN;
            }

            impl Max for $T {
                const MAX: Self = Self::MAX;
            }

            impl Nan for $T {
                const NAN: Self = Self::NAN;
            }

            impl Infinity for $T {
                const INFINITY: Self = Self::INFINITY;
            }

            impl NegInfinity for $T {
                const NEG_INFINITY: Self = Self::NEG_INFINITY;
            }
        )*
    };
}
impl_float!(f32, f64);

macro_rules! impl_int {
    ($($T:ty),*$(,)?) => {
        $(
            impl Zero for $T {
                const ZERO: Self = 0;
            }

            impl One for $T {
                const ONE: Self = 1;
            }

            impl NegOne for $T {
                const NEG_ONE: Self = -1;
            }

            impl Min for $T {
                const MIN: Self = Self::MIN;
            }

            impl Max for $T {
                const MAX: Self = Self::MAX;
            }
        )*
    };
}
impl_int!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_uint {
    ($($T:ty),*$(,)?) => {
        $(
            impl Zero for $T {
                const ZERO: Self = 0;
            }

            impl One for $T {
                const ONE: Self = 1;
            }

            impl Min for $T {
                const MIN: Self = Self::MIN;
            }

            impl Max for $T {
                const MAX: Self = Self::MAX;
            }
        )*
    };
}
impl_uint!(u8, u16, u32, u64, u128, usize);

impl True for bool {
    const TRUE: Self = true;
}

impl False for bool {
    const FALSE: Self = false;
}
