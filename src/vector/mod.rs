//! Vector related types and traits

use std::{
    fmt::{Debug, Display},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    slice::SliceIndex,
};

use crate::{Construct, Usize};

mod interface;

/// A generic vector type.
///
/// This type is generic over 3 parameters:
/// - `N`: The length of the vector, which currently supports 2, 3, and 4.
/// - `T`: The type of the vector, which must implement the [`Scalar`] trait.
/// - `A`: The "alignment" of the vector, which enables or disables SIMD memory alignment.
///
/// It is usually recommended to use vector type-aliases instead of the vector type directly.
/// `Vec{N}` type-aliases like `Vec2` are SIMD aligned and are considered the default.
/// `Vec{N}P` type-aliases like `Vec2P` are not SIMD aligned.
///
/// # Alignment
///
/// To understand alignment first see <https://doc.rust-lang.org/reference/type-layout.html>.
///
/// `A` must be either `VecAligned` or `VecPacked`.
/// A `VecAligned` vector is SIMD aligned while a `VecPacked` vector is not.
///
/// This means that `VecAligned` vectors result in faster operations than `VecPacked` vectors,
/// but `VecAligned` vectors have a larger size than `VecPacked` vectors.
/// When using a vector you need to choose between `VecAligned` and `VecPacked` based on priorities.
///
/// * `VecAligned` does not guarentee any specific alignment.
/// The alignment depends on the target architecture and the implementation of [`Scalar`].
///
/// * `VecPacked` guarentees that the vector has the exact same memory layout as `[T; N]`.
///
/// # Examples
/// ```
/// use ggmath::aliases::*;
///
/// // This is a non memory critical scenario so we should use `VecAligned`.
/// struct PlayerState {
///     // Vector<3, f32, VecAligned>
///     position: Vec3<f32>,
///     // ...
/// }
///
/// // This is a memory critical scenario so we should use `VecPacked`.
/// struct GpuVertex {
///     // Vector<3, f32, VecPacked>
///     position: Vec3P<f32>,
///     // Vector<3, f32, VecPacked>
///     normal: Vec3P<f32>,
///     // Vector<2, f32, VecPacked>
///     uv: Vec2P<f32>,
/// }
/// ```
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    /// The inner value that contains the actual vector data.
    ///
    /// For `VecAligned` vectors this will be `<T as Scalar>::InnerAlignedVec{N}`,
    /// like `<T as Scalar>::InnerAlignedVec2` for `Vec2`.
    ///
    /// For `VecPacked` vectors this will always be `[T; N]`.
    ///
    /// This field is public so that `Scalar` implementations that override vector function implementations
    /// can access the inner value.
    pub inner: A::InnerVector<N, T>,
}

/// A trait that marks a `Usize<N>` type as a valid vector length.
pub trait VecLen {
    /// The length value as an enum.
    const ENUM: VecLenEnum;

    /// The inner aligned vector type which will be:
    /// - `T::InnerAlignedVec2` for `Usize<2>`
    /// - `T::InnerAlignedVec3` for `Usize<3>`
    /// - `T::InnerAlignedVec4` for `Usize<4>`
    type InnerAlignedVector<T: Scalar>: Construct;
}

/// An enum with all currently supported vector lengths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecLenEnum {
    /// `2`
    Two,
    /// `3`
    Three,
    /// `4`
    Four,
}

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
    ) -> Vector<N, Self::Output, A>
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
    ) -> Vector<N, Self::Output, A>
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
        vector.map(|x| x * x).sum()
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
        Vector::<N, Self::Output, A>::from_fn(|i| vector[i] * other[i]).sum()
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
}

/// `Vector` is generic over `A: VecAlignment`,
/// which specifies if the vector is SIMD aligned or not.
///
/// This trait is implemented for `VecAligned` and `VecPacked`.
/// - `VecAligned` marks the vector as SIMD aligned.
/// - `VecPacked` marks the vector as not SIMD aligned.
///
/// See [`Vector`] alignment for more information.
pub trait VecAlignment: 'static {
    /// Whether the vector is SIMD aligned.
    const IS_ALIGNED: bool;

    /// The inner vector type based on `N` and `T`.
    /// This is the actual type that is stored in vectors.
    ///
    /// - `VecAligned` sets this to `T::InnerAlignedVec2` / `T::InnerAlignedVec3` / `T::InnerAlignedVec4`.
    /// - `VecPacked` sets this to `[T; N]`.
    type InnerVector<const N: usize, T: Scalar>: Construct
    where
        Usize<N>: VecLen;
}

/// Marks a `Vector` as SIMD aligned.
///
/// `Vector` can be marked as either `VecAligned` or `VecPacked`.
///
/// A `VecAligned` vector is aligned for optimal SIMD operations.
/// The exact type stored in aligned vectors is specified in the implementation of [`Scalar`].
///
/// A `VecPacked` vector is not aligned for SIMD operations.
/// It is guarenteed that a packed vector has the memory layout of `[T; N]`.
///
/// See [`Vector`] alignment for more information.
pub struct VecAligned;

/// Marks a `Vector` as not SIMD aligned.
///
/// `Vector` can be marked as either `VecAligned` or `VecPacked`.
///
/// A `VecAligned` vector is aligned for optimal SIMD operations.
/// The exact type stored in aligned vectors is specified in the implementation of [`Scalar`].
///
/// A `VecPacked` vector is not aligned for SIMD operations.
/// It is guarenteed that a packed vector has the memory layout of `[T; N]`.
///
/// See [`Vector`] alignment for more information.
pub struct VecPacked;

/// Constructs an aligned vec2 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, VecAligned>::from(($($expr),*,))
    };
}

/// Constructs an aligned vec3 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, VecAligned>::from(($($expr),*,))
    };
}

/// Constructs an aligned vec4 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4 {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, VecAligned>::from(($($expr),*,))
    };
}

/// Constructs a packed vec2 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a packed vec3 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a packed vec4 from the given values.
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4p {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, VecPacked>::from(($($expr),*,))
    };
}

/// Constructs a vec2 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 2.
/// This means that the options are:
/// - (scalar, scalar)
/// - (vec2)
#[macro_export]
macro_rules! vec2g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<2, _, _>::from(($($expr),*,))
    };
}

/// Constructs a vec3 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 3.
/// This means that the options are:
/// - (scalar, scalar, scalar)
/// - (vec2, scalar)
/// - (scalar, vec2)
/// - (vec3)
#[macro_export]
macro_rules! vec3g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<3, _, _>::from(($($expr),*,))
    };
}

/// Constructs a vec4 from the given values that needs type inference to decide if its [`VecAligned`] or [`VecPacked`].
///
/// This macro accepts any mix of scalars and vectors that sum up to a length of 4.
/// This means that the options are:
/// - (scalar, scalar, scalar, scalar)
/// - (vec2, scalar, scalar)
/// - (scalar, vec2, scalar)
/// - (scalar, scalar, vec2)
/// - (vec2, vec2)
/// - (vec3, scalar)
/// - (scalar, vec3)
/// - (vec4)
#[macro_export]
macro_rules! vec4g {
    ($($expr:expr),* $(,)?) => {
        $crate::vector::Vector::<4, _, _>::from(($($expr),*,))
    };
}

/// Creates a new aligned vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2<T: Scalar>(value: T) -> Vector<2, T, VecPacked> {
    Vector::<2, T, VecPacked>::splat(value)
}

/// Creates a new aligned vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3<T: Scalar>(value: T) -> Vector<3, T, VecPacked> {
    Vector::<3, T, VecPacked>::splat(value)
}

/// Creates a new aligned vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4<T: Scalar>(value: T) -> Vector<4, T, VecPacked> {
    Vector::<4, T, VecPacked>::splat(value)
}

/// Creates a new packed vec2 where each component is the same value.
#[inline(always)]
pub const fn splat2p<T: Scalar>(value: T) -> Vector<2, T, VecPacked> {
    Vector::<2, T, VecPacked>::splat(value)
}

/// Creates a new packed vec3 where each component is the same value.
#[inline(always)]
pub const fn splat3p<T: Scalar>(value: T) -> Vector<3, T, VecPacked> {
    Vector::<3, T, VecPacked>::splat(value)
}

/// Creates a new packed vec4 where each component is the same value.
#[inline(always)]
pub const fn splat4p<T: Scalar>(value: T) -> Vector<4, T, VecPacked> {
    Vector::<4, T, VecPacked>::splat(value)
}

/// Creates a new vec2 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat2g<T: Scalar, A: VecAlignment>(value: T) -> Vector<2, T, A> {
    Vector::<2, T, A>::splat(value)
}

/// Creates a new vec3 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat3g<T: Scalar, A: VecAlignment>(value: T) -> Vector<3, T, A> {
    Vector::<3, T, A>::splat(value)
}

/// Creates a new vec4 where each component is the same value,
/// where type inference can be used to determine if it's aligned or packed.
#[inline(always)]
pub const fn splat4g<T: Scalar, A: VecAlignment>(value: T) -> Vector<4, T, A> {
    Vector::<4, T, A>::splat(value)
}

impl VecLen for Usize<2> {
    const ENUM: VecLenEnum = VecLenEnum::Two;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec2;
}
impl VecLen for Usize<3> {
    const ENUM: VecLenEnum = VecLenEnum::Three;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec3;
}
impl VecLen for Usize<4> {
    const ENUM: VecLenEnum = VecLenEnum::Four;

    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}

impl VecAlignment for VecAligned {
    const IS_ALIGNED: bool = true;

    type InnerVector<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::InnerAlignedVector<T>
    where
        Usize<N>: VecLen;
}
impl VecAlignment for VecPacked {
    const IS_ALIGNED: bool = false;

    type InnerVector<const N: usize, T: Scalar>
        = [T; N]
    where
        Usize<N>: VecLen;
}

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{:?}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}
impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_eq(self, other)
    }

    #[inline(always)]
    fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_ne(self, other)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment, I: SliceIndex<[T]>> Index<I> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = I::Output;

    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_array()[index]
    }
}
impl<const N: usize, T: Scalar, A: VecAlignment, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_array_mut()[index]
    }
}

impl<const N: usize, T: Scalar + Neg<Output: Scalar>, A: VecAlignment> Neg for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        T::vec_neg(self)
    }
}

impl<const N: usize, T: Scalar + Not<Output: Scalar>, A: VecAlignment> Not for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        T::vec_not(self)
    }
}

impl<
    const N: usize,
    T: Scalar + Add<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Add<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_add(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Sub<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Sub<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_sub(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Mul<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Mul<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_mul(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Div<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Div<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_div(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Rem<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Rem<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_rem(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Shl<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Shl<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_shl(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + Shr<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> Shr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_shr(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitAnd<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitAnd<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitand(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitOr<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitOr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitor(self, other)
    }
}

impl<
    const N: usize,
    T: Scalar + BitXor<T2, Output: Scalar>,
    A: VecAlignment,
    T2: Scalar,
    A2: VecAlignment,
> BitXor<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, other: Vector<N, T2, A2>) -> Self::Output {
        T::vec_bitxor(self, other)
    }
}

impl<const N: usize, T: Scalar + AddAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    AddAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn add_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_add_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + SubAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    SubAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn sub_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_sub_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + MulAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    MulAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn mul_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_mul_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + DivAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    DivAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn div_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_div_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + RemAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    RemAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn rem_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_rem_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + ShlAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    ShlAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shl_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_shl_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + ShrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    ShrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shr_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_shr_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitAndAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitAndAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitand_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitand_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitOrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitOrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitor_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitor_assign(self, other)
    }
}

impl<const N: usize, T: Scalar + BitXorAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    BitXorAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, other: Vector<N, T2, A2>) {
        T::vec_bitxor_assign(self, other)
    }
}

impl<T: Scalar, A: VecAlignment> From<(T, T)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Self::from_array([value.0, value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>,)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>,)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2]])
    }
}
impl<T: Scalar, A: VecAlignment> From<(T, T, T, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2, value.3])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, T, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0, value.1, value.2[0], value.2[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<2, T, A0>, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A0>, T)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(T, Vector<3, T, A0>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T, A0>)) -> Self {
        Self::from_array([value.0, value.1[0], value.1[1], value.1[2]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, T, T)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, T, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1, value.2])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<2, T, A0>, Vector<2, T, A0>)>
    for Vector<4, T, A>
{
    #[inline(always)]
    fn from(value: (Vector<2, T, A0>, Vector<2, T, A0>)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.1[0], value.1[1]])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<3, T, A0>, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A0>, T)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.1])
    }
}
impl<T: Scalar, A: VecAlignment, A0: VecAlignment> From<(Vector<4, T, A0>,)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<4, T, A0>,)) -> Self {
        Self::from_array([value.0[0], value.0[1], value.0[2], value.0[3]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _verify_construct_impl<const N: usize, T: Scalar, A: VecAlignment>()
    where
        Usize<N>: VecLen,
    {
        fn helper<T: Construct>() {}

        helper::<Vector<N, T, A>>();
    }
}
