use std::{
    mem::{transmute, transmute_copy},
    ops::*,
};

use super::*;

// Length

/// Trait that marks a [`Usize`] value as a valid length for a vector.
///
/// This trait is implemented for `2`, `3` and `4`.
/// If this trait were to be implemented for an additional value, it would be a fully supported vector/matrix length/dimension.
///
/// In the future if rust has more powerful const-generics,
/// this trait could be deleted and all `usize` values will be valid vector lengths.
///
/// Implementing this trait for a custom type would do nothing since vectors require `Usize<N>: VecLen` specificly.
pub unsafe trait VecLen {
    /// Is used to "redirect" the vector to its alignment marker-type through this pattern:
    ///
    /// ```
    /// use ggmath::AlignTrait;
    ///
    /// trait VecAlignment {
    ///     type Alignment<const N: usize, T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait VecLen {
    ///     type Alignment<T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait Scalar {
    ///     type Vec2Alignment: AlignTrait;
    ///     type Vec3Alignment: AlignTrait;
    ///     type Vec4Alignment: AlignTrait;
    /// }
    /// ```
    type Alignment<T: Scalar>: AlignTrait;
}

unsafe impl VecLen for Usize<2> {
    type Alignment<T: Scalar> = T::Vec2Alignment;
}

unsafe impl VecLen for Usize<3> {
    type Alignment<T: Scalar> = T::Vec3Alignment;
}

unsafe impl VecLen for Usize<4> {
    type Alignment<T: Scalar> = T::Vec4Alignment;
}

// Scalar

/// Trait for types that can be put inside math-types like `Vector` and `Matrix`.
/// For example: `f32`, `u8` and `bool` are scalars.
///
/// All scalar types must be `Construct`,
/// which means `Copy` `'static` and that the value can be sent anywhere anytime.
///
/// References and vectors are NOT scalars.
///
/// ### Implementing this trait
///
/// When implementing this trait, you need to fill out the [`VecAligned`] alignments for your type.
///
/// ```
/// use ggmath::*;
///
/// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// struct MyF32(f32);
///
/// impl Scalar for MyF32 {
///     type Vec2Alignment = Align<8>;
///     type Vec3Alignment = Align<16>;
///     type Vec4Alignment = Align<16>;
/// }
/// ```
///
/// `ggmath` currently trusts that rustc will use the alignment to correctly use SIMD instructions.
/// This means you currently cannot override the implementations of built in functions like operators.
pub trait Scalar: Construct {
    /// Controls the alignment of `Vec2<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec2Alignment: AlignTrait;

    /// Controls the alignment of `Vec3<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec3Alignment: AlignTrait;

    /// Controls the alignment of `Vec4<Self>`.
    /// This will be the applied alignment only if the vector type is `VecAligned`.
    type Vec4Alignment: AlignTrait;

    // Swizzle

    /// Overridable implementation of swizzle functions that return a `Vec2`,
    /// like [`Vec2::xy`] and [`Vec3::yz`].
    #[inline(always)]
    fn to_vec2_swizzle<const N: usize, A: VecAlignment, const X: usize, const Y: usize>(
        vec: Vector<N, Self, A>,
    ) -> Vector<2, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec2g!(vec[X], vec[Y])
    }

    /// Overridable implementation of swizzle functions that return a `Vec3`,
    /// like [`Vec2::xyy`] and [`Vec4::www`].
    #[inline(always)]
    fn to_vec3_swizzle<
        const N: usize,
        A: VecAlignment,
        const X: usize,
        const Y: usize,
        const Z: usize,
    >(
        vec: Vector<N, Self, A>,
    ) -> Vector<3, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec3g!(vec[X], vec[Y], vec[Z])
    }

    /// Overridable implementation of swizzle functions that return a `Vec4`,
    /// like [`Vec2::xyxy`] and [`Vec3::yzyx`].
    #[inline(always)]
    fn to_vec4_swizzle<
        const N: usize,
        A: VecAlignment,
        const X: usize,
        const Y: usize,
        const Z: usize,
        const W: usize,
    >(
        vec: Vector<N, Self, A>,
    ) -> Vector<4, Self, A>
    where
        Usize<N>: VecLen,
    {
        vec4g!(vec[X], vec[Y], vec[Z], vec[W])
    }

    // Comparison

    /// Overridable implementation of [`Vector::eq_mask`].
    #[inline(always)]
    fn vec_eq_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        vec.map_rhs(rhs, |x, y| x == y)
    }

    /// Overridable implementation of [`Vector::ne_mask`].
    #[inline(always)]
    fn vec_ne_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq_mask(vec, rhs)
    }

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
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
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        !Self::vec_eq(vec, rhs)
    }

    /// Overridable implementation of [`Vector::lt_mask`].
    #[inline(always)]
    fn vec_lt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        vec.map_rhs(rhs, |x, y| x < y)
    }

    /// Overridable implementation of [`Vector::gt_mask`].
    #[inline(always)]
    fn vec_gt_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        vec.map_rhs(rhs, |x, y| x > y)
    }

    /// Overridable implementation of [`Vector::le_mask`].
    #[inline(always)]
    fn vec_le_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        vec.map_rhs(rhs, |x, y| x <= y)
    }

    /// Overridable implementation of [`Vector::ge_mask`].
    #[inline(always)]
    fn vec_ge_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd<T2>,
    {
        vec.map_rhs(rhs, |x, y| x >= y)
    }

    /// Overridable implementation of [`Vector::min`].
    #[inline(always)]
    fn vec_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd,
    {
        vec.map_rhs(rhs, |x, y| if x < y { x } else { y })
    }

    /// Overridable implementation of [`Vector::max`].
    #[inline(always)]
    fn vec_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd,
    {
        vec.map_rhs(rhs, |x, y| if x > y { x } else { y })
    }

    /// Overridable implementation of [`Vector::clamp`].
    #[inline(always)]
    fn vec_clamp<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        min: Vector<N, Self, impl VecAlignment>,
        max: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd,
    {
        vec.max(min).min(max)
    }

    // Unary Ops

    /// Negates a vector.
    /// This is used to implement [`Neg`] for [`Vector`].
    #[inline(always)]
    fn vec_neg<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vec.map(|x| -x)
    }

    /// Inverts a vector.
    /// This is used to implement [`Not`] for [`Vector`].
    #[inline(always)]
    fn vec_not<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vec.map(|x| !x)
    }

    // Binary Ops

    /// Adds two vectors.
    /// This is used to implement [`Add`] for [`Vector`].
    #[inline(always)]
    fn vec_add<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Add<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x + y)
    }

    /// Subtracts two vectors.
    /// This is used to implement [`Sub`] for [`Vector`].
    #[inline(always)]
    fn vec_sub<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Sub<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x - y)
    }

    /// Multiplies two vectors.
    /// This is used to implement [`Mul`] for [`Vector`].
    #[inline(always)]
    fn vec_mul<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Mul<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x * y)
    }

    /// Divides two vectors.
    /// This is used to implement [`Div`] for [`Vector`].
    #[inline(always)]
    fn vec_div<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Div<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x / y)
    }

    /// Takes the remainder of two vectors.
    /// This is used to implement [`Rem`] for [`Vector`].
    #[inline(always)]
    fn vec_rem<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Rem<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x % y)
    }

    /// Shifts the bits of a vector to the left.
    /// This is used to implement [`Shl`] for [`Vector`].
    #[inline(always)]
    fn vec_shl<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shl<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x << y)
    }

    /// Shifts the bits of a vector to the right.
    /// This is used to implement [`Shr`] for [`Vector`].
    #[inline(always)]
    fn vec_shr<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: Shr<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x >> y)
    }

    /// Performs a bitwise AND on two vectors.
    /// This is used to implement [`BitAnd`] for [`Vector`].
    #[inline(always)]
    fn vec_bitand<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitAnd<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x & y)
    }

    /// Performs a bitwise OR on two vectors.
    /// This is used to implement [`BitOr`] for [`Vector`].
    #[inline(always)]
    fn vec_bitor<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitOr<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x | y)
    }

    /// Performs a bitwise XOR on two vectors.
    /// This is used to implement [`BitXor`] for [`Vector`].
    #[inline(always)]
    fn vec_bitxor<const N: usize, A: VecAlignment, T2: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        Usize<N>: VecLen,
        Self: BitXor<T2, Output: Scalar>,
    {
        vec.map_rhs(rhs, |x, y| x ^ y)
    }

    // Operator Dependent Functions

    /// Overridable implementation of [`Vector::sum`].
    #[inline(always)]
    fn vec_sum<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        Usize<N>: VecLen,
        Self: Add<Output = Self>,
    {
        vec.fold(|a, b| a + b)
    }

    /// Overridable implementation of [`Vector::dot`].
    #[inline(always)]
    fn vec_dot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        Usize<N>: VecLen,
        Self: Add<Output = Self> + Mul<Output = Self>,
    {
        (vec * other).sum()
    }

    /// Overridable implementation of [`Vector::cross`].
    #[inline(always)]
    fn vec_cross<A: VecAlignment>(
        vec: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self> + Sub<Output = Self>,
    {
        (vec.zxy() * other - vec * other.zxy()).zxy()
    }

    /// Overridable implementation of [`Vector::abs_diff`].
    #[inline(always)]
    fn vec_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        Usize<N>: VecLen,
        Self: PartialOrd + Sub<Output = Self>,
    {
        vec.map_rhs(rhs, |x, y| if x > y { x - y } else { y - x })
    }
}

// Alignment

/// Sealed marker trait.
///
/// All `ggmath` types are generic over the *marker* type `A: VecAlignment`.
///
/// The value of `A` decides whether or not the vector is aligned for SIMD (see <https://doc.rust-lang.org/reference/type-layout.html>).
///
/// This marker trait is implemented by `VecAligned` and `VecPacked`.
///
/// `VecAligned` means that the vector is aligned for SIMD.
/// This does not promise a specific alignment.
/// The alignment is selected by the `Scalar` implementation,
/// to be whatever is most efficient for the target platform's SIMD.
///
/// `VecPacked` means that the vector is not aligned for SIMD, and is identical in memory to `[T; N]`.
///
/// Implementing this trait for a custom type is straight up undefined behavior.
#[allow(private_bounds)]
pub unsafe trait VecAlignment: Sized + 'static + Send + Sync {
    /// Is used internally by [`Vector`] to know if the vector is aligned in a generic function.
    const IS_ALIGNED: bool;

    /// Is used to "redirect" the vector to its alignment marker-type through this pattern:
    ///
    /// ```
    /// use ggmath::AlignTrait;
    ///
    /// trait VecAlignment {
    ///     type Alignment<const N: usize, T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait VecLen {
    ///     type Alignment<T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait Scalar {
    ///     type Vec2Alignment: AlignTrait;
    ///     type Vec3Alignment: AlignTrait;
    ///     type Vec4Alignment: AlignTrait;
    /// }
    /// ```
    type Alignment<const N: usize, T: Scalar>: AlignTrait
    where
        Usize<N>: VecLen;
}

/// See [`VecAlignment`].
pub struct VecAligned;

/// See [`VecAlignment`].
pub struct VecPacked;

unsafe impl VecAlignment for VecAligned {
    const IS_ALIGNED: bool = true;

    type Alignment<const N: usize, T: Scalar>
        = <Usize<N> as VecLen>::Alignment<T>
    where
        Usize<N>: VecLen;
}

unsafe impl VecAlignment for VecPacked {
    const IS_ALIGNED: bool = false;

    type Alignment<const N: usize, T: Scalar>
        = Align<1>
    where
        Usize<N>: VecLen;
}

// Resolve

/// A vector enum that is split based on memory-layout generics.
///
/// This is used to implement memory-layout generic functions by "matching" the generic arguments:
///
/// ```
/// use ggmath::*;
///
/// fn example<A: VecAlignment>(vector: Vector<2, f32, A>) {
///     match vector.resolve() {
///         ResolvedVector::Aligned(aligned) => {
///             println!("{aligned} is aligned");
///         }
///         ResolvedVector::Packed(packed) => {
///             println!("{packed} is packed");
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVector<const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    /// The vector is aligned.
    Aligned(Vector<N, T, VecAligned>),
    /// The vector is packed.
    Packed(Vector<N, T, VecPacked>),
}

/// A vector reference enum that is split based on memory-layout generics.
///
/// This is used to implement memory-layout generic functions by "matching" the generic arguments:
///
/// ```
/// use ggmath::*;
///
/// fn example<A: VecAlignment>(vector: &Vector<2, f32, A>) {
///     match vector.resolve_ref() {
///         ResolvedVectorRef::Aligned(aligned) => {
///             println!("{aligned} is aligned");
///         }
///         ResolvedVectorRef::Packed(packed) => {
///             println!("{packed} is packed");
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVectorRef<'a, const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    /// The vector is aligned.
    Aligned(&'a Vector<N, T, VecAligned>),
    /// The vector is packed.
    Packed(&'a Vector<N, T, VecPacked>),
}

/// A vector mutable reference enum that is split based on memory-layout generics.
///
/// This is used to implement memory-layout generic functions by "matching" the generic arguments:
///
/// ```
/// use ggmath::*;
///
/// fn example<A: VecAlignment>(vector: &mut Vector<2, f32, A>) {
///     match vector.resolve_mut() {
///         ResolvedVectorMut::Aligned(aligned) => {
///             println!("{aligned} is aligned");
///         }
///         ResolvedVectorMut::Packed(packed) => {
///             println!("{packed} is packed");
///         }
///     }
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedVectorMut<'a, const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    /// The vector is aligned.
    Aligned(&'a mut Vector<N, T, VecAligned>),
    /// The vector is packed.
    Packed(&'a mut Vector<N, T, VecPacked>),
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Resolves the vector into a [`ResolvedVector`] enum.
    ///
    /// This is used to implement memory-layout generic functions by "matching" the generic arguments.
    ///
    /// ```
    /// use ggmath::*;
    ///
    /// fn example<A: VecAlignment>(vector: Vector<2, f32, A>) {
    ///     match vector.resolve() {
    ///         ResolvedVector::Aligned(aligned) => {
    ///             println!("{aligned} is aligned");
    ///         }
    ///         ResolvedVector::Packed(packed) => {
    ///             println!("{packed} is packed");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedVector<N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVector::Aligned(
                    transmute_copy::<Vector<N, T, A>, Vector<N, T, VecAligned>>(&self),
                )
            } else {
                ResolvedVector::Packed(transmute_copy::<Vector<N, T, A>, Vector<N, T, VecPacked>>(
                    &self,
                ))
            }
        }
    }

    /// Resolves the vector reference into a [`ResolvedVectorRef`] enum.
    ///
    /// This is used to implement memory-layout generic functions by "matching" the generic arguments.
    ///
    /// ```
    /// use ggmath::*;
    ///
    /// fn example<A: VecAlignment>(vector: &Vector<2, f32, A>) {
    ///     match vector.resolve_ref() {
    ///         ResolvedVectorRef::Aligned(aligned) => {
    ///             println!("{aligned} is aligned");
    ///         }
    ///         ResolvedVectorRef::Packed(packed) => {
    ///             println!("{packed} is packed");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_ref(&'_ self) -> ResolvedVectorRef<'_, N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVectorRef::Aligned(
                    transmute::<&Vector<N, T, A>, &Vector<N, T, VecAligned>>(self),
                )
            } else {
                ResolvedVectorRef::Packed(transmute::<&Vector<N, T, A>, &Vector<N, T, VecPacked>>(
                    self,
                ))
            }
        }
    }

    /// Resolves the vector mutable reference into a [`ResolvedVectorMut`] enum.
    ///
    /// This is used to implement memory-layout generic functions by "matching" the generic arguments.
    ///
    /// ```
    /// use ggmath::*;
    ///
    /// fn example<A: VecAlignment>(vector: &mut Vector<2, f32, A>) {
    ///     match vector.resolve_mut() {
    ///         ResolvedVectorMut::Aligned(aligned) => {
    ///             println!("{aligned} is aligned");
    ///         }
    ///         ResolvedVectorMut::Packed(packed) => {
    ///             println!("{packed} is packed");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_mut(&'_ mut self) -> ResolvedVectorMut<'_, N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVectorMut::Aligned(transmute::<
                    &mut Vector<N, T, A>,
                    &mut Vector<N, T, VecAligned>,
                >(self))
            } else {
                ResolvedVectorMut::Packed(transmute::<
                    &mut Vector<N, T, A>,
                    &mut Vector<N, T, VecPacked>,
                >(self))
            }
        }
    }

    /// "unresolves" the vector by selecting the value with the correct memory-layout generics.
    ///
    /// This trusts rustc to do dead-code elimination to only compute the value that is needed.
    #[inline(always)]
    pub const fn resolved(
        aligned: Vector<N, T, VecAligned>,
        packed: Vector<N, T, VecPacked>,
    ) -> Self {
        if A::IS_ALIGNED {
            unsafe { transmute_copy::<Vector<N, T, VecAligned>, Vector<N, T, A>>(&aligned) }
        } else {
            unsafe { transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(&packed) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!(vec2!(1, 2).resolve(), ResolvedVector::Aligned(vec2!(1, 2)));
        assert_eq!(
            vec3p!(1, 2, 3).resolve(),
            ResolvedVector::Packed(vec3p!(1, 2, 3))
        );

        assert_eq!(
            vec2!(1, 2).resolve_ref(),
            ResolvedVectorRef::Aligned(&vec2!(1, 2))
        );
        assert_eq!(
            vec3p!(1, 2, 3).resolve_ref(),
            ResolvedVectorRef::Packed(&vec3p!(1, 2, 3))
        );

        assert_eq!(
            vec2!(1, 2).resolve_mut(),
            ResolvedVectorMut::Aligned(&mut vec2!(1, 2))
        );
        assert_eq!(
            vec3p!(1, 2, 3).resolve_mut(),
            ResolvedVectorMut::Packed(&mut vec3p!(1, 2, 3))
        );

        assert_eq!(Vec2::resolved(vec2!(1, 2), vec2p!(3, 4)), vec2!(1, 2));
        assert_eq!(
            Vec3P::resolved(vec3!(1, 2, 3), vec3p!(3, 4, 5)),
            vec3!(3, 4, 5)
        );
    }
}
