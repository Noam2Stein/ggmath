//! Traits for scalar constants like `ZERO`, `ONE` and `NAN`.
//!
//! Each constant has its own trait which when implemented enables the constant
//! for vectors.
//!
//! # Example
//!
//! Lets define our own scalar type (see [`Scalar`]):
//!
//! ```
//! use ggmath::{Scalar, ScalarDefault};
//!
//! #[derive(Debug, Clone, Copy)]
//! struct Foo(f32);
//!
//! impl Scalar for Foo {}
//!
//! impl ScalarDefault for Foo {}
//! ```
//!
//! Lets implement the traits [`Zero`], [`One`], and [`NegOne`] for it:
//!
//! ```
//! # use ggmath::{Scalar, ScalarDefault};
//! #
//! # #[derive(Debug, Clone, Copy)]
//! # struct Foo(f32);
//! #
//! # impl Scalar for Foo {}
//! #
//! # impl ScalarDefault for Foo {}
//! #
//! use ggmath::constants::{NegOne, One, Zero};
//!
//! impl Zero for Foo {
//!     const ZERO: Self = Self(0.0);
//! }
//!
//! impl One for Foo {
//!     const ONE: Self = Self(1.0);
//! }
//!
//! impl NegOne for Foo {
//!     const NEG_ONE: Self = Self(-1.0);
//! }
//! ```
//!
//! Now we can use vector constants `ZERO`, `ONE`, and `NEG_ONE` plus
//! directional constants like `X`, `Y`, `NEG_X`, and `NEG_Y`:
//!
//! ```
//! # use ggmath::{Scalar, ScalarDefault};
//! #
//! # #[derive(Debug, Clone, Copy)]
//! # struct Foo(f32);
//! #
//! # impl Scalar for Foo {}
//! #
//! # impl ScalarDefault for Foo {}
//! #
//! # use ggmath::constants::{NegOne, One, Zero};
//! #
//! # impl Zero for Foo {
//! #     const ZERO: Self = Self(0.0);
//! # }
//! #
//! # impl One for Foo {
//! #     const ONE: Self = Self(1.0);
//! # }
//! #
//! # impl NegOne for Foo {
//! #     const NEG_ONE: Self = Self(-1.0);
//! # }
//! #
//! use ggmath::Vec2;
//!
//! println!("{:?}", Vec2::<Foo>::ZERO);
//! println!("{:?}", Vec2::<Foo>::ONE);
//! println!("{:?}", Vec2::<Foo>::NEG_ONE);
//! println!("{:?}", Vec2::<Foo>::X);
//! println!("{:?}", Vec2::<Foo>::Y);
//! println!("{:?}", Vec2::<Foo>::NEG_X);
//! println!("{:?}", Vec2::<Foo>::NEG_Y);
//! ```
//!
//! You can also write generic code where you know `T` has some constant:
//!
//! ```
//! # use ggmath::{Scalar, ScalarDefault};
//! #
//! # #[derive(Debug, Clone, Copy)]
//! # struct Foo(f32);
//! #
//! # impl Scalar for Foo {}
//! #
//! # impl ScalarDefault for Foo {}
//! #
//! # use ggmath::constants::{NegOne, One, Zero};
//! #
//! # impl Zero for Foo {
//! #     const ZERO: Self = Self(0.0);
//! # }
//! #
//! # impl One for Foo {
//! #     const ONE: Self = Self(1.0);
//! # }
//! #
//! # impl NegOne for Foo {
//! #     const NEG_ONE: Self = Self(-1.0);
//! # }
//! #
//! use ggmath::Vec2;
//!
//! fn print_zero<T: Scalar + core::fmt::Debug + Zero>() {
//!     println!("{:?}", Vec2::<T>::ZERO);
//! }
//!
//! print_zero::<Foo>();
//! ```

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
    /// The smallest value that can be represented by this type.
    const MIN: Self;
}

/// A `MAX` constant for scalar types.
pub trait Max: Scalar {
    /// The largest value that can be represented by this type.
    const MAX: Self;
}

/// A `NAN` constant for scalar types.
pub trait Nan: Scalar {
    /// NaN (Not a Number).
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
