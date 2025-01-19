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
    crate::scalar_defaults_vector_splat! {}
    crate::scalar_defaults_vector_get! {}
    crate::scalar_defaults_vector_with! {}

    // ********************************************************************************
    // ********************************************************************************
    // ************************************* STD **************************************
    // ********************************************************************************
    // ********************************************************************************

    #[inline(always)]
    fn vector_eq<const N: usize, TRhs: Scalar>(
        vec: &Vector<N, Self, impl VecAlignment>,
        other: &Vector<N, TRhs, impl VecAlignment>,
    ) -> bool
    where
        ScalarCount<N>: VecLen,
        Self: PartialEq<TRhs>,
    {
        (0..N).all(|i| vec[i].eq(&other[i]))
    }

    #[inline(always)]
    fn vector_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: Default,
    {
        Vector::splat(Self::default())
    }

    // Vector: Self Ops

    fn vector_neg<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vec.map(|c| c.neg())
    }

    fn vector_not<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vec.map(|c| c.not())
    }

    // Vector: Rhs Ops

    fn vector_add<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Add<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].add(rhs[i]))
    }

    fn vector_bitand<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitAnd<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitand(rhs[i]))
    }

    fn vector_bitor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitOr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitor(rhs[i]))
    }

    fn vector_bitxor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitXor<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitxor(rhs[i]))
    }

    fn vector_div<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Div<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].div(rhs[i]))
    }

    fn vector_mul<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Mul<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].mul(rhs[i]))
    }

    fn vector_rem<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Rem<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].rem(rhs[i]))
    }

    fn vector_shl<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shl<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shl(rhs[i]))
    }

    fn vector_shr<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shr(rhs[i]))
    }

    fn vector_sub<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Sub<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].sub(rhs[i]))
    }

    // Vector: Assign Ops

    fn vector_add_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: AddAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].add_assign(rhs[i]);
        }
    }

    fn vector_bitand_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitAndAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitand_assign(rhs[i]);
        }
    }

    fn vector_bitor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitOrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitor_assign(rhs[i]);
        }
    }

    fn vector_bitxor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitXorAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitxor_assign(rhs[i]);
        }
    }

    fn vector_div_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: DivAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].div_assign(rhs[i]);
        }
    }

    fn vector_mul_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: MulAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].mul_assign(rhs[i]);
        }
    }

    fn vector_rem_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: RemAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].rem_assign(rhs[i]);
        }
    }

    fn vector_shl_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShlAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shl_assign(rhs[i]);
        }
    }

    fn vector_shr_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shr_assign(rhs[i]);
        }
    }

    fn vector_sub_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: SubAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].sub_assign(rhs[i]);
        }
    }

    // ********************************************************************************
    // ********************************************************************************
    // ********** CRATE TRAITS (that should probably be in std to be honest) **********
    // ********************************************************************************
    // ********************************************************************************

    splat_attribs! {
        #[cfg(feature = "num")]:

        // Vector: Round

        #[inline(always)]
        fn vector_round<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self::Output, A>
        where
            Self: Round<Output: Scalar>,
            ScalarCount<N>: VecLen,
        {
            vec.map(Round::round)
        }

        #[inline(always)]
        fn vector_floor<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self::Output, A>
        where
            Self: Round<Output: Scalar>,
            ScalarCount<N>: VecLen,
        {
            vec.map(Round::floor)
        }

        #[inline(always)]
        fn vector_ceil<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self::Output, A>
        where
            Self: Round<Output: Scalar>,
            ScalarCount<N>: VecLen,
        {
            vec.map(Round::ceil)
        }

        #[inline(always)]
        fn vector_trunc<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self::Output, A>
        where
            Self: Round<Output: Scalar>,
            ScalarCount<N>: VecLen,
        {
            vec.map(Round::trunc)
        }

        #[inline(always)]
        fn vector_atrunc<const N: usize, A: VecAlignment>(
            vec: Vector<N, Self, A>,
        ) -> Vector<N, Self::Output, A>
        where
            Self: Round<Output: Scalar>,
            ScalarCount<N>: VecLen,
        {
            vec.map(Round::atrunc)
        }
    }

    // Vector: Trig

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_sin<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::sin)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cos<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::cos)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_tan<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::tan)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::cot)
    }

    // Vector: ATrig

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_asin<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::asin)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_acos<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::acos)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_atan<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::atan)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_acot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::acot)
    }

    // Vector: TrigH

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_sinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::sinh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_cosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::cosh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_tanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::tanh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_coth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::coth)
    }

    // Vector: ATrigH

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_asinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::asinh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_acosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::acosh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_atanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::atanh)
    }

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_acoth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::acoth)
    }

    // Vector: AbsDiff

    #[cfg(feature = "num")]
    #[inline(always)]
    fn vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: AbsDiff<Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].abs_diff(rhs[i]))
    }

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
