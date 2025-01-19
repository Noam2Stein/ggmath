//! Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
//! For example: [```f32```], [```u8```] and [```bool```] are scalars.

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use splat_attribs::splat_attribs;

use crate::{vector::*, *};

splat_attribs! {
    #[cfg(feature = "num")]:

    use newnum::{ATrig, ATrigH, AbsDiff, Cbrt, Round, Sqrt, Trig, TrigH};
    use std::cmp::Ordering;
}

mod wrapper;
pub use wrapper::*;

pub use ggmath_proc_macros::scalar_inner_vectors;

mod primitive_impls;

/// Specifies the inner aligned-vector types for a scalar type,
/// because Rust doesn't have a type which is generic over alignment.
///
/// Required by ```Scalar```.
/// Use the [```scalar_inner_vectors```] macro to implement this correctly.
pub unsafe trait ScalarInnerAlignedVecs {
    type InnerAlignedVec2: Construct;
    type InnerAlignedVec4: Construct;
}

/// Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
/// For example: [```f32```], [```u8```] and [```bool```] are scalars.
///
/// - References are NOT scalars.
///
/// ### Implementing this trait
///
/// To implement this trait for a type that wraps another ```Scalar``` type
/// (for example ```struct Meters(f32);```),
/// use the inexistant wrapper system.
///
/// To implement this trait for a unique type use this example:
///
/// ```
/// struct u256(u128, u128);
///
/// scalar_inner_vectors!(u256(32)); // 32 - size in bytes
///
/// impl Scalar for u256 {}
/// ```
pub trait Scalar: Construct + ScalarInnerAlignedVecs {
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ********************************************** Scalar **********************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************
    // ****************************************************************************************************

    splat_attribs! {
        #[inline(always)]:

        // Min Max Clamp

        fn min(self, other: Self) -> Self
        where
            Self: PartialOrd,
        {
            if self > other {
                other
            } else {
                self
            }
        }

        fn max(self, other: Self) -> Self
        where
            Self: PartialOrd,
        {
            if self > other {
                self
            } else {
                other
            }
        }

        fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: PartialOrd,
        {
            if self > max {
                max
            } else if self < min {
                min
            } else {
                self
            }
        }
    }

    // **************************************************
    // ********************* Vector *********************
    // **************************************************

    // Core
    scalar_defaults_vector_splat! {}
    scalar_defaults_vector_get! {}
    scalar_defaults_vector_with! {}

    // STD
    scalar_defaults_vector_eq! {}
    scalar_defaults_vector_default! {}
    scalar_defaults_vector_ops! {}

    // Num
    splat_attribs! {
        #[cfg(feature = "num")]:

        scalar_defaults_vector_round! {}
        scalar_defaults_vector_sign! {}
        scalar_defaults_vector_trig! {}
        scalar_defaults_vector_abs_diff! {}
    }

    // ********************************************************************************
    // ********************************************************************************
    // ********** CRATE TRAITS (that should probably be in std to be honest) **********
    // ********************************************************************************
    // ********************************************************************************

    // Vector: Sqrt

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_sqrt<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Sqrt<Output: Scalar>,
    {
        vec.map(Sqrt::sqrt)
    }

    // Vector: Cbrt

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cbrt<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Cbrt<Output: Scalar>,
    {
        vec.map(Cbrt::cbrt)
    }

    // ********************************************************************************
    // ********************************************************************************
    // ************************************* API **************************************
    // ********************************************************************************
    // ********************************************************************************

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_csum<const N: usize>(vec: Vector<N, Self, impl VecAlignment>) -> Self
    where
        Self: Add<Output = Self>,
        ScalarCount<N>: VecLen,
    {
        match vec.resolve_length() {
            LengthResolvedVector::Vec2(vec) => vec.x() + vec.y(),
            LengthResolvedVector::Vec3(vec) => vec.x() + vec.y() + vec.z(),
            LengthResolvedVector::Vec4(vec) => vec.x() + vec.y() + vec.z() + vec.w(),
        }
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_dot<const N: usize>(
        vec: Vector<N, Self, impl VecAlignment>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        Self: Mul<Output = Self> + Add<Output = Self>,
        ScalarCount<N>: VecLen,
    {
        (vec * other).csum()
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cross<A: VecAlignment>(
        vec: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self> + Sub<Output = Self>,
    {
        (vec.zxy() * other - vec * other.zxy()).zxy()
    }

    // Vector: Min Max

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cmin<const N: usize>(vec: Vector<N, Self, impl VecAlignment>) -> Self
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        vec.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cmax<const N: usize>(vec: Vector<N, Self, impl VecAlignment>) -> Self
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        vec.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        Vector::from_fn(|i| match vec[i].partial_cmp(&other[i]) {
            None => vec[i],
            Some(Ordering::Less) => vec[i],
            Some(Ordering::Equal) => vec[i],
            Some(Ordering::Greater) => other[i],
        })
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        Vector::from_fn(|i| match vec[i].partial_cmp(&other[i]) {
            None => vec[i],
            Some(Ordering::Less) => other[i],
            Some(Ordering::Equal) => vec[i],
            Some(Ordering::Greater) => vec[i],
        })
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_clamp<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        min: Vector<N, Self, impl VecAlignment>,
        max: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        vec.max(min).min(max)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! scalar_defaults_macro {
    ($macro_ident:ident: $($tt:tt)*) => {
        #[macro_export(local_inner_macros)]
        macro_rules! $macro_ident {
            () => {
                $($tt)*
            }
        }
    };
}
