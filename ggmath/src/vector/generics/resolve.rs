use std::mem::{transmute, transmute_copy};

use super::*;

/// A vector enum that is split based on memory-layout generics.
///
/// This is used to implement memory-layout generic functions by "matching" the generic arguments:
///
/// ```
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
