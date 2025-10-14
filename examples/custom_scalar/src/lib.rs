//! This example declares a custom scalar type and enables SIMD optimizations for its vector.

#![deny(missing_docs)]

use std::ops::{Add, Sub};

use ggmath::*;

/// A fixed-point number type with 8 fractional bits.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FixedPoint(i32);

// Generate vector type-aliases for `FixedPoint` (like `FPVec2 => Vector<2, FixedPoint, Simd>`).
declare_vector_aliases!(pub type FP => FixedPoint);

impl FixedPoint {
    /// Creates a new `FixedPoint` from an `i32`.
    pub const fn from_i32(value: i32) -> Self {
        Self(value << 8)
    }

    /// Creates a new `FixedPoint` from a `f32`.
    pub const fn from_f32(value: f32) -> Self {
        Self((value * 256.0) as i32)
    }

    /// Converts the `FixedPoint` to a `f32`.
    pub const fn as_f32(self) -> f32 {
        self.0 as f32 / 256.0
    }
}

impl Add for FixedPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // To add two `FixedPoint`s, we simply add the underlying `i32`s.
        Self(self.0 + other.0)
    }
}

impl Sub for FixedPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        // To subtract two `FixedPoint`s, we simply subtract the underlying `i32`s.
        Self(self.0 - other.0)
    }
}

// These implementations automatically add `ZERO`/`ONE`/`NEG_ONE` constants to `FixedPoint` vectors,
// as well as `RIGHT`/`LEFT`/`UP`/`DOWN`/`FORWARDS`/`BACKWARDS` constants.

impl ScalarZero for FixedPoint {
    const ZERO: Self = Self(0);
}

impl ScalarOne for FixedPoint {
    const ONE: Self = Self::from_i32(1);
}

impl ScalarNegOne for FixedPoint {
    const NEG_ONE: Self = Self::from_i32(-1);
}

// These implementations allow us to use `FixedPoint`s in vectors.
// To add SIMD optimizations, we use `i32` vectors internally because they are already optimized.

// SAFETY:
// We directly wrap the appropriate `i32` vector type and padding.
// This is sound because `FixedPoint` is `repr(transparent)` around `i32`.
impl<const N: usize> Scalar<N> for FixedPoint
where
    i32: Scalar<N>,
{
    type InnerSimdVectorType = Vector<N, i32, Simd>;

    // Override the implementation of `Add` for `FixedPoint` vectors to use SIMD.
    fn vec_add(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Add<Output = Self>,
    {
        // <FixedPoint as Add> - "To add two `FixedPoint`s, we simply add the underlying `i32`s."
        //
        // This means we can also simply add the underlying `i32` vectors to add two `FixedPoint` vectors.
        // * `i32` vector addition is already optimized for SIMD.
        Vector(vec.0 + rhs.0)
    }

    // From here we can override the implementation of more functions to use SIMD.
}

#[cfg(test)]
mod tests {
    use ggmath::{ScalarZero, vec2};

    use crate::{FPVec2, FixedPoint};

    #[test]
    fn example_usage() {
        // Now we can use `FixedPoint` in vectors, and perform operations on them.

        let a = vec2!(FixedPoint::ZERO, FixedPoint::from_i32(2));
        let b = FPVec2::X;

        // Because `FixedPoint` implements `Sub`, so do `FixedPoint` vectors.
        // Beware that because we didn't optimize `sub`, its not SIMD optimized.
        let _ = a + b - vec2!(FixedPoint::from_i32(3));
    }
}

unsafe impl TransmuteTo<i32> for FixedPoint {}
