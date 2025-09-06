use crate::{Construct, Scalar, Usize, VecLen};

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
