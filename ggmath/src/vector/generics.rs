use std::mem::{transmute, transmute_copy};

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

    /// Allows you to optimize the implementation of `Vec3<Self>::neg`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_neg(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = base;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::not`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_not(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = base;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::add`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::sub`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::mul`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::div`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_div(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::rem`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_rem(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::shl`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_shl(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::shr`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_shr(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitand`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitor`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
    }

    /// Allows you to optimize the implementation of `Vec3<Self>::bitxor`.
    ///
    /// The implementation is meant to always return either `Some` or `None`.
    /// If you return `None`, the default implementation will be used.
    /// If you return `Some`, your implementation will be used.
    ///
    /// Operator optimization functions are only available for `Vec3<Self>`.
    /// This is because usually only vector3s have padding,
    /// which stops the compiler from using SIMD instructions on its own.
    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let _ = lhs;
        let _ = rhs;

        None
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
    pub const fn resolve_ref(&self) -> ResolvedVectorRef<N, T> {
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
    pub const fn resolve_mut(&mut self) -> ResolvedVectorMut<N, T> {
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
