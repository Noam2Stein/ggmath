use std::ops::*;

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
/// # Overriding vector functions
///
/// When implementing `Scalar` you can override the implementation most vector functions.
/// This is what allows `ggmath` to be both heavily generic and a zero-cost abstraction.
///
/// * Overriding vector functions is ONLY for optimizations.
/// Overrides should never change the behavior of functions.
///
/// The overridable functions usually have alot of generics
/// and require unsafe type-matching and transmuting to specialize per vector length/alignment.
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
///     type InnerAlignedVec2 = std::arch::x86_64::__m128i;
///     type InnerAlignedVec3 = std::arch::x86_64::__m128i;
///     type InnerAlignedVec4 = std::arch::x86_64::__m128i;
///
///     const GARBAGE: Self = Self(0);
///     const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = std::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = std::arch::x86_64::_mm_setzero_si128();
///     const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = std::arch::x86_64::_mm_setzero_si128();
///     
///     // override the implementation of vector addition using simd
///     #[inline(always)]
///     fn vec_add<const N: usize, A: VecAlignment, T2: Scalar>(
///         vector: Vector<N, Self, A>,
///         other: Vector<N, T2, impl VecAlignment>,
///     ) -> Vector<N, <Self as Add<T2>>::Output, A>
///     where
///         Usize<N>: VecLen,
///         Self: Add<T2, Output: Scalar>,
///     {
///         #[cfg(debug_assertions)]
///         {
///             Vector::from_fn(|i| vector[i] + other[i])
///         }
///
///         #[cfg(not(debug_assertions))]
///         {
///             if std::any::TypeId::of::<T2>() != std::any::TypeId::of::<Self>() || !A::IS_ALIGNED {
///                 return Vector::from_fn(|i| vector[i] + other[i]);
///             }
///
///             match <Usize<N> as VecLen>::ENUM {
///                 VecLenEnum::Two => {
///                     let vector = std::mem::transmute::<Vector<N, Self, A>, Vector<2, Self, VecAligned>>(vector);
///                     let other = std::mem::transmute::<Vector<N, T2, VecAligned>, Vector<2, Self, VecAligned>>(other);
///
///                     let output = Vector::<2, Self, VecAligned> {
///                         inner: std::arch::x86_64::_mm_add_epi32(vector.inner, other.inner),
///                     };
///                     
///                     std::mem::transmute::<Vector<2, Self, VecAligned>, Vector<N, Self::Output, A>>(output)
///                 }
///                 VecLenEnum::Three => {
///                     let vector = std::mem::transmute::<Vector<N, Self, A>, Vector<3, Self, VecAligned>>(vector);
///                     let other = std::mem::transmute::<Vector<N, T2, VecAligned>, Vector<3, Self, VecAligned>>(other);
///
///                     let output = Vector::<3, Self, VecAligned> {
///                         inner: std::arch::x86_64::_mm_add_epi32(vector.inner, other.inner),
///                     };
///
///                     std::mem::transmute::<Vector<3, Self, VecAligned>, Vector<N, Self::Output, A>>(output)
///                 }
///                 VecLenEnum::Four => {
///                     let vector = std::mem::transmute::<Vector<N, Self, A>, Vector<4, Self, VecAligned>>(vector);
///                     let other = std::mem::transmute::<Vector<N, T2, VecAligned>, Vector<4, Self, VecAligned>>(other);
///
///                     let output = Vector::<4, Self, VecAligned> {
///                         inner: std::arch::x86_64::_mm_add_epi32(vector.inner, other.inner),
///                     };
///
///                     std::mem::transmute::<Vector<4, Self, VecAligned>, Vector<N, Self::Output, A>>(output)
///                 }
///             }
///         }
///     }
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

    /// Overridable implementation of vector swizzling functions that return a vec2.
    /// For example `Vector::yz` calls `vec_swizzle2::<_, _, 1, 2>`.
    ///
    /// It is guarenteed that the given indicies are in bounds.
    #[inline(always)]
    fn vec_swizzle2<const N: usize, A: VecAlignment, const SRC0: usize, const SRC1: usize>(
        vector: Vector<N, Self, A>,
    ) -> Vector<2, Self, A>
    where
        Usize<N>: VecLen,
    {
        Vector::<2, _, _>::from_array([vector[SRC0], vector[SRC1]])
    }

    /// Overridable implementation of vector swizzling functions that return a vec3.
    /// For example `Vector::wyy` calls `vec_swizzle3::<_, _, 3, 1, 1>`.
    ///
    /// It is guarenteed that the given indicies are in bounds.
    #[inline(always)]
    fn vec_swizzle3<
        const N: usize,
        A: VecAlignment,
        const SRC0: usize,
        const SRC1: usize,
        const SRC2: usize,
    >(
        vector: Vector<N, Self, A>,
    ) -> Vector<3, Self, A>
    where
        Usize<N>: VecLen,
    {
        Vector::<3, _, _>::from_array([vector[SRC0], vector[SRC1], vector[SRC2]])
    }

    /// Overridable implementation of vector swizzling functions that return a vec4.
    /// For example `Vector::wxyz` calls `vec_swizzle4::<_, _, 3, 0, 1, 2>`.
    ///
    /// It is guarenteed that the given indicies are in bounds.
    #[inline(always)]
    fn vec_swizzle4<
        const N: usize,
        A: VecAlignment,
        const SRC0: usize,
        const SRC1: usize,
        const SRC2: usize,
        const SRC3: usize,
    >(
        vector: Vector<N, Self, A>,
    ) -> Vector<4, Self, A>
    where
        Usize<N>: VecLen,
    {
        Vector::<4, _, _>::from_array([vector[SRC0], vector[SRC1], vector[SRC2], vector[SRC3]])
    }

    /// Overridable implementation of `Vector::eq`.
    #[inline(always)]
    fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        for i in 0..N {
            if vector[i] != other[i] {
                return false;
            }
        }

        true
    }

    /// Overridable implementation of `Vector::ne`.
    #[inline(always)]
    fn vec_ne<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq(vector, other)
    }

    /// Overridable implementation of `Vector::neg`.
    #[inline(always)]
    fn vec_neg<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
    ) -> Vector<N, <Self as Neg>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vector.map(|x| -x)
    }

    /// Overridable implementation of `Vector::not`.
    #[inline(always)]
    fn vec_not<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
    ) -> Vector<N, <Self as Not>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vector.map(|x| !x)
    }

    /// Overridable implementation of `Vector::add`.
    #[inline(always)]
    fn vec_add<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Add<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Add<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] + other[i])
    }

    /// Overridable implementation of `Vector::sub`.
    #[inline(always)]
    fn vec_sub<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Sub<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Sub<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] - other[i])
    }

    /// Overridable implementation of `Vector::mul`.
    #[inline(always)]
    fn vec_mul<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Mul<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] * other[i])
    }

    /// Overridable implementation of `Vector::div`.
    #[inline(always)]
    fn vec_div<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Div<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Div<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] / other[i])
    }

    /// Overridable implementation of `Vector::rem`.
    #[inline(always)]
    fn vec_rem<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Rem<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Rem<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] % other[i])
    }

    /// Overridable implementation of `Vector::shl`.
    #[inline(always)]
    fn vec_shl<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Shl<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shl<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] << other[i])
    }

    /// Overridable implementation of `Vector::shr`.
    #[inline(always)]
    fn vec_shr<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as Shr<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] >> other[i])
    }

    /// Overridable implementation of `Vector::bitand`.
    #[inline(always)]
    fn vec_bitand<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitAnd<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitAnd<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] & other[i])
    }

    /// Overridable implementation of `Vector::bitor`.
    #[inline(always)]
    fn vec_bitor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitOr<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitOr<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] | other[i])
    }

    /// Overridable implementation of `Vector::bitxor`.
    #[inline(always)]
    fn vec_bitxor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as BitXor<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitXor<T2, Output: Scalar>,
    {
        Vector::from_fn(|i| vector[i] ^ other[i])
    }

    /// Overridable implementation of `Vector::add_assign`.
    #[inline(always)]
    fn vec_add_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: AddAssign<T2>,
    {
        for i in 0..N {
            vector[i] += other[i];
        }
    }

    /// Overridable implementation of `Vector::sub_assign`.
    #[inline(always)]
    fn vec_sub_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: SubAssign<T2>,
    {
        for i in 0..N {
            vector[i] -= other[i];
        }
    }

    /// Overridable implementation of `Vector::mul_assign`.
    #[inline(always)]
    fn vec_mul_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: MulAssign<T2>,
    {
        for i in 0..N {
            vector[i] *= other[i];
        }
    }

    /// Overridable implementation of `Vector::div_assign`.
    #[inline(always)]
    fn vec_div_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: DivAssign<T2>,
    {
        for i in 0..N {
            vector[i] /= other[i];
        }
    }

    /// Overridable implementation of `Vector::rem_assign`.
    #[inline(always)]
    fn vec_rem_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: RemAssign<T2>,
    {
        for i in 0..N {
            vector[i] %= other[i];
        }
    }

    /// Overridable implementation of `Vector::shl_assign`.
    #[inline(always)]
    fn vec_shl_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShlAssign<T2>,
    {
        for i in 0..N {
            vector[i] <<= other[i];
        }
    }

    /// Overridable implementation of `Vector::shr_assign`.
    #[inline(always)]
    fn vec_shr_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: ShrAssign<T2>,
    {
        for i in 0..N {
            vector[i] >>= other[i];
        }
    }

    /// Overridable implementation of `Vector::bitand_assign`.
    #[inline(always)]
    fn vec_bitand_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitAndAssign<T2>,
    {
        for i in 0..N {
            vector[i] &= other[i];
        }
    }

    /// Overridable implementation of `Vector::bitor_assign`.
    #[inline(always)]
    fn vec_bitor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitOrAssign<T2>,
    {
        for i in 0..N {
            vector[i] |= other[i];
        }
    }

    /// Overridable implementation of `Vector::bitxor_assign`.
    #[inline(always)]
    fn vec_bitxor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: BitXorAssign<T2>,
    {
        for i in 0..N {
            vector[i] ^= other[i];
        }
    }

    /// Overridable implementation of `Vector::eq_mask`.
    #[inline(always)]
    fn vec_eq_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        Vector::from_fn(|i| vector[i] == other[i])
    }

    /// Overridable implementation of `Vector::ne_mask`.
    #[inline(always)]
    fn vec_ne_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq_mask(vector, other)
    }

    /// Overridable implementation of `Vector::lt_mask`.
    #[inline(always)]
    fn vec_lt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vector[i] < other[i])
    }

    /// Overridable implementation of `Vector::le_mask`.
    #[inline(always)]
    fn vec_le_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vector[i] <= other[i])
    }

    /// Overridable implementation of `Vector::gt_mask`.
    #[inline(always)]
    fn vec_gt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vector[i] > other[i])
    }

    /// Overridable implementation of `Vector::ge_mask`.
    #[inline(always)]
    fn vec_ge_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        Vector::from_fn(|i| vector[i] >= other[i])
    }

    /// Overridable implementation of `Vector::sum`.
    #[inline(always)]
    fn vec_sum<const N: usize, A: VecAlignment>(vector: Vector<N, Self, A>) -> Self
    where
        Usize<N>: VecLen,
        Self: Add<Output = Self>,
    {
        vector.fold(|a, b| a + b)
    }

    /// Overridable implementation of `Vector::product`.
    #[inline(always)]
    fn vec_product<const N: usize, A: VecAlignment>(vector: Vector<N, Self, A>) -> Self
    where
        Usize<N>: VecLen,
        Self: Mul<Output = Self>,
    {
        vector.fold(|a, b| a * b)
    }

    /// Overridable implementation of `Vector::mag_sq`.
    #[inline(always)]
    fn vec_mag_sq<const N: usize, A: VecAlignment>(vector: Vector<N, Self, A>) -> Self
    where
        Usize<N>: VecLen,
        Self: Add<Output = Self> + Mul<Output = Self>,
    {
        (vector * vector).sum()
    }

    /// Overridable implementation of `Vector::dot`.
    #[inline(always)]
    fn vec_dot<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Self::Output
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
        Self::Output: Add<Output = Self::Output>,
    {
        (vector * other).sum()
    }

    /// Overridable implementation of `Vector::cross`.
    #[inline(always)]
    fn vec_cross<A: VecAlignment>(
        vector: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A>
    where
        Self: Mul<Self, Output = Self>,
        Self: Sub<Self, Output = Self>,
    {
        vector.yzx() * other.zxy() - vector.zxy() * other.yzx()
    }

    /// Overridable implementation of `Vector::abs_diff`.
    #[inline(always)]
    fn vec_abs_diff<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd + Sub<Output = Self>,
    {
        Vector::<N, Self, A>::from_fn(|i| {
            if vector[i] < other[i] {
                other[i] - vector[i]
            } else {
                vector[i] - other[i]
            }
        })
    }

    /// Overridable implementation of `Vector::distance_sq`.
    #[inline(always)]
    fn vec_distance_sq<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        Usize<N>: VecLen,
        Self: PartialOrd + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>,
    {
        vector.abs_diff(other).mag_sq()
    }
}
