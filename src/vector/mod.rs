//! Module for the vector type.

use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::*,
};

use super::*;

mod cmp;
mod construct;
mod convert;
mod index;
mod iter;
mod ops;
mod splat;
mod swizzle;
pub use construct::*;
pub use splat::*;

/// Marks a [`Usize`] type as a valid length for a vector.
/// This trait is currently implemented for `2`, `3` and `4`.
///
/// Note that more lengths like `5` could be made valid and it would NOT be a breaking change.
pub unsafe trait VecLen {
    /// Specifies the inner memory representation of an aligned vector.
    ///
    /// This currently always returns one of `T`'s inner vector types.
    type InnerVecA<T: Scalar>: Construct;
}

unsafe impl VecLen for Usize<2> {
    type InnerVecA<T: Scalar> = T::InnerVec2A;
}

unsafe impl VecLen for Usize<3> {
    type InnerVecA<T: Scalar> = T::InnerVec3A;
}

unsafe impl VecLen for Usize<4> {
    type InnerVecA<T: Scalar> = T::InnerVec4A;
}

/// Allows types to be put inside math-types like `Vector` and `Matrix`.
/// For example: `f32`, `u8` and `bool` are scalars.
///
/// All scalar types are `Copy` and some more basic traits like `Send` and `Sync`.
pub trait Scalar: Construct {
    /// Specifies the inner memory representation of an aligned vector2.
    /// This type's reference must be a valid `&[T; 2]`.
    ///
    /// This type is useful to add extra memory alignment to vectors for use in SIMD instructions.
    /// If `Self` does not benifit from SIMD or additional memory alignment, use `[Self; 2]` to minimize size.
    type InnerVec2A: Construct;

    /// Specifies the inner memory representation of an aligned vector3.
    /// This type's reference must be a valid `&[T; 3]`.
    ///
    /// This type is useful to add extra memory alignment to vectors for use in SIMD instructions.
    /// If `Self` does not benifit from SIMD or additional memory alignment, use `[Self; 3]` to minimize size.
    type InnerVec3A: Construct;

    /// Specifies the inner memory representation of an aligned vector4.
    /// This type's reference must be a valid `&[T; 4]`.
    ///
    /// This type is useful to add extra memory alignment to vectors for use in SIMD instructions.
    /// If `Self` does not benifit from SIMD or additional memory alignment, use `[Self; 4]` to minimize size.
    type InnerVec4A: Construct;

    /// Specifies a valid value of the `InnerVec2A` type.
    ///
    /// This is used when initializing vectors to make sure padding is initialized.
    const INNER_VEC2A_GARBAGE: Self::InnerVec2A;

    /// Specifies a valid value of the `InnerVec3A` type.
    ///
    /// This is used when initializing vectors to make sure padding is initialized.
    const INNER_VEC3A_GARBAGE: Self::InnerVec3A;

    /// Specifies a valid value of the `InnerVec4A` type.
    ///
    /// This is used when initializing vectors to make sure padding is initialized.
    const INNER_VEC4A_GARBAGE: Self::InnerVec4A;

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        Vector::from_fn(|i| vec[i].eq(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        Vector::from_fn(|i| vec[i].ne(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::lt`].
    #[inline(always)]
    fn vec_lt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vec[i].lt(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::le`].
    #[inline(always)]
    fn vec_le_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vec[i].le(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::gt`].
    #[inline(always)]
    fn vec_gt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vec[i].gt(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::ge`].
    #[inline(always)]
    fn vec_ge_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vec[i].ge(&rhs[i]))
    }

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        Self::vec_eq_mask(vec, rhs).into_iter().all(|x| x)
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq(vec, rhs)
    }

    /// Overridable implementation of [`Vector::neg`].
    #[inline(always)]
    fn vec_neg<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        Vector::from_array(vec.to_array().map(|x| x.neg()))
    }

    /// Overridable implementation of [`Vector::not`].
    #[inline(always)]
    fn vec_not<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        Vector::from_array(vec.to_array().map(|x| x.not()))
    }

    /// Overridable implementation of [`Vector::add`].
    #[inline(always)]
    fn vec_add<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Add<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].add(rhs[i]))
    }

    /// Overridable implementation of [`Vector::add_assign`].
    #[inline(always)]
    fn vec_add_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: AddAssign<T2>,
    {
        for i in 0..N {
            vec[i].add_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::sub`].
    #[inline(always)]
    fn vec_sub<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Sub<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].sub(rhs[i]))
    }

    /// Overridable implementation of [`Vector::sub_assign`].
    #[inline(always)]
    fn vec_sub_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: SubAssign<T2>,
    {
        for i in 0..N {
            vec[i].sub_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::mul`].
    #[inline(always)]
    fn vec_mul<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].mul(rhs[i]))
    }

    /// Overridable implementation of [`Vector::mul_assign`].
    #[inline(always)]
    fn vec_mul_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: MulAssign<T2>,
    {
        for i in 0..N {
            vec[i].mul_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::div`].
    #[inline(always)]
    fn vec_div<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Div<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].div(rhs[i]))
    }

    /// Overridable implementation of [`Vector::div_assign`].
    #[inline(always)]
    fn vec_div_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: DivAssign<T2>,
    {
        for i in 0..N {
            vec[i].div_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::rem`].
    #[inline(always)]
    fn vec_rem<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Rem<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].rem(rhs[i]))
    }

    /// Overridable implementation of [`Vector::rem_assign`].
    #[inline(always)]
    fn vec_rem_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: RemAssign<T2>,
    {
        for i in 0..N {
            vec[i].rem_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::shl`].
    #[inline(always)]
    fn vec_shl<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shl<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shl(rhs[i]))
    }

    /// Overridable implementation of [`Vector::shl_assign`].
    #[inline(always)]
    fn vec_shl_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShlAssign<T2>,
    {
        for i in 0..N {
            vec[i].shl_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::shr`].
    #[inline(always)]
    fn vec_shr<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shr(rhs[i]))
    }

    /// Overridable implementation of [`Vector::shr_assign`].
    #[inline(always)]
    fn vec_shr_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShrAssign<T2>,
    {
        for i in 0..N {
            vec[i].shr_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::bitand`].
    #[inline(always)]
    fn vec_bitand<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitAnd<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitand(rhs[i]))
    }

    /// Overridable implementation of [`Vector::bitand_assign`].
    #[inline(always)]
    fn vec_bitand_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitAndAssign<T2>,
    {
        for i in 0..N {
            vec[i].bitand_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::bitor`].
    #[inline(always)]
    fn vec_bitor<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitOr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitor(rhs[i]))
    }

    /// Overridable implementation of [`Vector::bitor_assign`].
    #[inline(always)]
    fn vec_bitor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitOrAssign<T2>,
    {
        for i in 0..N {
            vec[i].bitor_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::bitxor`].
    #[inline(always)]
    fn vec_bitxor<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitXor<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitxor(rhs[i]))
    }

    /// Overridable implementation of [`Vector::bitxor_assign`].
    #[inline(always)]
    fn vec_bitxor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitXorAssign<T2>,
    {
        for i in 0..N {
            vec[i].bitxor_assign(rhs[i]);
        }
    }

    /// Overridable implementation of [`Vector::mul`].
    #[inline(always)]
    fn vec_scalar_mul<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: T2,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].mul(rhs))
    }

    /// Overridable implementation of [`Vector::mul_assign`].
    #[inline(always)]
    fn vec_scalar_mul_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: T2,
    ) where
        Usize<N>: VecLen,
        Self: MulAssign<T2>,
    {
        for i in 0..N {
            vec[i].mul_assign(rhs);
        }
    }

    /// Overridable implementation of [`Vector::div`].
    #[inline(always)]
    fn vec_scalar_div<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: T2,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Div<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].div(rhs))
    }

    /// Overridable implementation of [`Vector::div_assign`].
    #[inline(always)]
    fn vec_scalar_div_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: T2,
    ) where
        Usize<N>: VecLen,
        Self: DivAssign<T2>,
    {
        for i in 0..N {
            vec[i].div_assign(rhs);
        }
    }

    /// Overridable implementation of [`Vector::rem`].
    #[inline(always)]
    fn vec_scalar_rem<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: T2,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Rem<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].rem(rhs))
    }

    /// Overridable implementation of [`Vector::rem_assign`].
    #[inline(always)]
    fn vec_scalar_rem_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: &mut Vector<N, Self, A>,
        rhs: T2,
    ) where
        Usize<N>: VecLen,
        Self: RemAssign<T2>,
    {
        for i in 0..N {
            vec[i].rem_assign(rhs);
        }
    }

    /// Overridable implementation of [`Vector::hash`].
    #[inline(always)]
    fn vec_hash<const N: usize, A: VecAlignment, H: std::hash::Hasher>(
        vec: &Vector<N, Self, A>,
        state: &mut H,
    ) where
        Usize<N>: VecLen,
        Self: std::hash::Hash,
    {
        vec.as_array_ref().hash(state);
    }

    /// Overridable implementation of [`Vector::default`].
    #[inline(always)]
    fn vec_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: Default,
    {
        Vector::splat(Self::default())
    }
}

/// Sealed marker trait that marks vectors as either `VecAligned` or `VecPacked`.
///
/// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
/// means the vector will have additional memory alignment to `T` if it improves performance,
/// like with SIMD instructions.
///
/// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
/// means the vector will not have additional memory alignment to `T`,
/// and will be identical in memory to `[T; N]`.
///
/// The `Vec{N}` type aliases like `Vec2` are for aligned vectors.
/// The `Vec{N}P` type aliases like `Vec2P` are for packed vectors.
pub unsafe trait VecAlignment: Sized + 'static + Send + Sync {
    /// Can be used to branch code based on the generic alignment of a vector.
    const IS_ALIGNED: bool;

    /// The inner memory representation of vectors.
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

/// Vectors can be marked as either `VecAligned` or `VecPacked`.
///
/// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
/// means the vector will have additional memory alignment to `T` if it improves performance,
/// like with SIMD instructions.
///
/// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
/// means the vector will not have additional memory alignment to `T`,
/// and will be identical in memory to `[T; N]`.
pub struct VecAligned;

/// Vectors can be marked as either `VecAligned` or `VecPacked`.
///
/// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
/// means the vector will have additional memory alignment to `T` if it improves performance,
/// like with SIMD instructions.
///
/// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
/// means the vector will not have additional memory alignment to `T`,
/// and will be identical in memory to `[T; N]`.
pub struct VecPacked;

unsafe impl VecAlignment for VecAligned {
    const IS_ALIGNED: bool = true;

    type InnerVector<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::InnerVecA<T>
    where
        Usize<N>: VecLen;
}

unsafe impl VecAlignment for VecPacked {
    const IS_ALIGNED: bool = false;

    type InnerVector<const N: usize, T: Scalar>
        = [T; N]
    where
        Usize<N>: VecLen;
}

/// The vector type.
///
/// This type has alot of generics, and most of the time type aliases should be used instead of this type.
/// See [`Vec2`], [`Vec3`] and [`Vec4`].
///
/// This type is generic over three parameters:
/// - `N`: The length of the vector. Can be 2, 3 or 4.
/// - `T`: The scalar type of the vector. Must implement [`Scalar`].
/// - `A`: The alignment of the vector. Specifies if the vector is aligned for SIMD, or unaligned (saves space).
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub inner: A::InnerVector<N, T>,
}

/// Type alias to `Vector<2, T, VecAligned>`.
///
/// This type is a vector2 that is aligned for SIMD instructions and is considered the default vector2 type.
/// For an unaligned vector2 that follows the memory layout of `[T; 2]`, see [`Vec2P`].
#[cfg(feature = "aliases")]
pub type Vec2<T> = Vector<2, T, VecAligned>;

/// Type alias to `Vector<2, T, VecPacked>`.
///
/// This type is a vector2 that follows the memory layout of `[T; 2]`.
/// As a result it has slower operations than [`Vec2`] but saves space.
#[cfg(feature = "aliases")]
pub type Vec2P<T> = Vector<2, T, VecPacked>;

/// Type alias to `Vector<3, T, VecAligned>`.
///
/// This type is a vector3 that is aligned for SIMD instructions and is considered the default vector3 type.
/// For an unaligned vector3 that follows the memory layout of `[T; 3]`, see [`Vec3P`].
#[cfg(feature = "aliases")]
pub type Vec3<T> = Vector<3, T, VecAligned>;

/// Type alias to `Vector<3, T, VecPacked>`.
///
/// This type is a vector3 that follows the memory layout of `[T; 3]`.
/// As a result it has slower operations than [`Vec3`] but saves space.
#[cfg(feature = "aliases")]
pub type Vec3P<T> = Vector<3, T, VecPacked>;

/// Type alias to `Vector<4, T, VecAligned>`.
///
/// This type is a vector4 that is aligned for SIMD instructions and is considered the default vector4 type.
/// For an unaligned vector4 that follows the memory layout of `[T; 4]`, see [`Vec4P`].
#[cfg(feature = "aliases")]
pub type Vec4<T> = Vector<4, T, VecAligned>;

/// Type alias to `Vector<4, T, VecPacked>`.
///
/// This type is a vector4 that follows the memory layout of `[T; 4]`.
/// As a result it has slower operations than [`Vec4`] but saves space.
#[cfg(feature = "aliases")]
pub type Vec4P<T> = Vector<4, T, VecPacked>;

/// Expands to a declaration of type specific vector aliases.
///
/// Syntax:
/// `<vis> <prefix> => <type>`
///
/// Example:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// vector_aliases!(pub Big => BigInt);
/// ```
///
/// Expands to:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// pub type BigVec2 = Vec2<BigInt>;
/// pub type BigVec3 = Vec3<BigInt>;
/// pub type BigVec4 = Vec4<BigInt>;
///
/// pub type BigVec2P = Vec2P<BigInt>;
/// pub type BigVec3P = Vec3P<BigInt>;
/// pub type BigVec4P = Vec4P<BigInt>;
/// ```
#[cfg(feature = "vector")]
#[macro_export]
macro_rules! vector_aliases {
    (pub($($vis:tt)*) $prefix:ident => $type:ident) => {
        $crate::vector_aliases! { @(pub($($vis)*)) $prefix => $type }
    };
    (pub $prefix:ident => $type:ident) => {
        $crate::vector_aliases! { @(pub) $prefix => $type }
    };
    ($prefix:ident => $type:ident) => {
        $crate::vector_aliases! { @() $prefix => $type }
    };

    (@($($vis:tt)*) $prefix:ident => $type:ident) => {
        $crate::_hidden_::paste! {
            #[doc = "Type alias to `Vec2<" $type ">`"]
			$($vis)* type [<$prefix Vec2>] = $crate::Vec2<$type>;

			#[doc = "Type alias to `Vec2P<" $type ">`"]
			$($vis)* type [<$prefix Vec2P>] = $crate::Vec2P<$type>;

			#[doc = "Type alias to `Vec3<" $type ">`"]
			$($vis)* type [<$prefix Vec3>] = $crate::Vec3<$type>;

			#[doc = "Type alias to `Vec3P<" $type ">`"]
			$($vis)* type [<$prefix Vec3P>] = $crate::Vec3P<$type>;

			#[doc = "Type alias to `Vec4<" $type ">`"]
			$($vis)* type [<$prefix Vec4>] = $crate::Vec4<$type>;

			#[doc = "Type alias to `Vec4P<" $type ">`"]
			$($vis)* type [<$prefix Vec4P>] = $crate::Vec4P<$type>;
        }
    };
}

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::vec_hash(self, state);
    }
}

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        T::vec_default()
    }
}

impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for item in &self.as_array_ref()[..N - 1] {
            write!(f, "{item}, ")?;
        }
        write!(f, "{}", self.as_array_ref()[N - 1])?;

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for item in &self.as_array_ref()[..N - 1] {
            write!(f, "{item:?}, ")?;
        }
        write!(f, "{:?}", self.as_array_ref()[N - 1])?;

        write!(f, ")")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // verify size
    const _: () = assert!(size_of::<Vector<2, f32, VecAligned>>() >= 8);
    const _: () = assert!(size_of::<Vector<3, f32, VecAligned>>() >= 12);
    const _: () = assert!(size_of::<Vector<4, f32, VecAligned>>() >= 16);
    const _: () = assert!(size_of::<Vector<2, f32, VecPacked>>() == 8);
    const _: () = assert!(size_of::<Vector<3, f32, VecPacked>>() == 12);
    const _: () = assert!(size_of::<Vector<4, f32, VecPacked>>() == 16);

    const _: () = assert!(size_of::<Vector<2, bool, VecAligned>>() >= 2);
    const _: () = assert!(size_of::<Vector<3, bool, VecAligned>>() >= 3);
    const _: () = assert!(size_of::<Vector<4, bool, VecAligned>>() >= 4);
    const _: () = assert!(size_of::<Vector<2, bool, VecPacked>>() == 2);
    const _: () = assert!(size_of::<Vector<3, bool, VecPacked>>() == 3);
    const _: () = assert!(size_of::<Vector<4, bool, VecPacked>>() == 4);

    // verify alignment
    const _: () = assert!(align_of::<Vector<2, f32, VecAligned>>() >= align_of::<f32>());
    const _: () = assert!(align_of::<Vector<3, f32, VecAligned>>() >= align_of::<f32>());
    const _: () = assert!(align_of::<Vector<4, f32, VecAligned>>() >= align_of::<f32>());
    const _: () = assert!(align_of::<Vector<2, f32, VecPacked>>() == align_of::<f32>());
    const _: () = assert!(align_of::<Vector<3, f32, VecPacked>>() == align_of::<f32>());
    const _: () = assert!(align_of::<Vector<4, f32, VecPacked>>() == align_of::<f32>());

    const _: () = assert!(align_of::<Vector<2, bool, VecAligned>>() >= align_of::<bool>());
    const _: () = assert!(align_of::<Vector<3, bool, VecAligned>>() >= align_of::<bool>());
    const _: () = assert!(align_of::<Vector<4, bool, VecAligned>>() >= align_of::<bool>());
    const _: () = assert!(align_of::<Vector<2, bool, VecPacked>>() == align_of::<bool>());
    const _: () = assert!(align_of::<Vector<3, bool, VecPacked>>() == align_of::<bool>());
    const _: () = assert!(align_of::<Vector<4, bool, VecPacked>>() == align_of::<bool>());

    // verify Construct
    fn _verify_vector_construct<const N: usize, T: Scalar, A: VecAlignment>()
    where
        Usize<N>: VecLen,
    {
        fn _verify_construct<T: Construct>() {}

        _verify_construct::<Vector<N, T, A>>();
    }
}
