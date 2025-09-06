use core::ops::Sub;

use crate::{Construct, Usize, VecAlignment, VecLen, Vector};

/// A trait that marks a type as a valid scalar type that can be used in a vector.
/// This trait is implemented for most primitive types, like `f32`, `f64`, `bool`, `usize`, etc.
///
/// # Implementing `Scalar`
///
/// When implementing `Scalar` you need to fill:
///
/// - `InnerAlignedVec2`, `InnerAlignedVec3`, and `InnerAlignedVec4`
/// are the inner types inside `VecAligned` vectors.
/// Their reference MUST be transmutable to `&[T; N]`,
/// if its not then using that vector is undefined behavior.
///
/// - `GARBAGE`, `INNER_ALIGNED_VEC2_GARBAGE`, `INNER_ALIGNED_VEC3_GARBAGE`, and `INNER_ALIGNED_VEC4_GARBAGE`
/// need to be any valid value of `Self`, `Self::InnerAlignedVec2`, `Self::InnerAlignedVec3`, or `Self::InnerAlignedVec4` respectively.
///
/// # Examples
///
/// ```ignore
/// struct HeapInt {
///     // private fields
/// }
///
/// // impl Add, Sub... for HeapInt
///
/// // lets say HeapInt cannot benefit from SIMD operations, or we just don't want to optimize yet.
/// // When not wanting SIMD we can fill `InnerAlignedVec{N}` with `[Self; N]`.
/// impl Scalar for HeapInt {
///     type InnerAlignedVec2 = [Self; 2];
///     type InnerAlignedVec3 = [Self; 3];
///     type InnerAlignedVec4 = [Self; 4];
///
///     const GARBAGE: Self = Self::ZERO;
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [Self::ZERO; 2];
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [Self::ZERO; 3];
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [Self::ZERO; 4];
/// }
///
/// struct MyScalar(i32);
///
/// // impl Add, Sub... for MyScalar
///
/// // lets say MyScalar can benefit from SIMD operations.
/// impl Scalar for MyScalar {
///     // use x86_64 simd types for aligned vectors.
///     type InnerAlignedVec2 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec3 = core::arch::x86_64::__m128i;
///     type InnerAlignedVec4 = core::arch::x86_64::__m128i;
///
///     const GARBAGE: Self = Self(0);
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = core::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = core::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = core::arch::x86_64::_mm_setzero_si128();
/// }
/// ```
pub trait Scalar: Construct {
    /// The inner type of `Vector<2, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 2]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec2: Construct;
    /// The inner type of `Vector<3, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 3]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec3: Construct;
    /// The inner type of `Vector<4, Self, VecAligned>`.
    /// This type's reference MUST be transmutable to `[Self; 4]`.
    /// Not following this rule will cause undefined behavior when using the vector.
    type InnerAlignedVec4: Construct;

    /// Any valid value of `Self`.
    /// This is sometimes used to create temporary values.
    const GARBAGE: Self;
    /// Any valid value of `Self::InnerAlignedVec2`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2;
    /// Any valid value of `Self::InnerAlignedVec3`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3;
    /// Any valid value of `Self::InnerAlignedVec4`.
    /// This is used to properly initialize aligned vector padding.
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4;

    /// Overridable implementation of `Vector::abs_diff`.
    #[inline(always)]
    fn vec_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd + Sub<Output = Self>,
    {
        Vector::from_fn(|i| {
            if vec[i] < other[i] {
                other[i] - vec[i]
            } else {
                vec[i] - other[i]
            }
        })
    }
}
