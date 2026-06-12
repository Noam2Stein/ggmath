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
        )*
    };
}
impl_uint!(u8, u16, u32, u64, u128, usize);
