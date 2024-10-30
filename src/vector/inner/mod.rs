//! Behaviour for selecting an inner-vector type based on a vector's length, scalar, and storage.
//!
//!

use super::*;

/// The type of the inner-value inside a vector
pub type InnerVector<const N: usize, T, S> = <S as VecAlignmentInnerVecs>::InnerVec<N, T>;

/// Scalar supertrait that specifies inner-types for vectors that can't be declared generically.
///
/// - Unsafe to implement manually because the implementation is expected to comply with type-layout guarentees.
/// Instead, implement using [```aligned_vecs```].
pub unsafe trait ScalarInnerVecs: Construct {
    /// Inner-type for ```VecAligned``` Vec2s.
    /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 2```
    type InnerAlignedVec2: InnerConstruct;

    /// Inner-type for ```VecAligned``` Vec4s and Vec3s.
    /// - Guarenteed: ```size = align = size_of::<T>().next_power_of_two() * 4```
    type InnerAlignedVec4: InnerConstruct;
}

pub use ggmath_proc_macros::inner_vecs;

#[doc(hidden)]
#[allow(private_bounds)]
pub trait VecLenInnerVec: Seal {
    type InnerAlignedVec<T: ScalarInnerVecs>: InnerConstruct;
}
impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLenInnerVec for ScalarCount<2> {
    type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec2;
}
impl VecLenInnerVec for ScalarCount<3> {
    type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
}
impl VecLenInnerVec for ScalarCount<4> {
    type InnerAlignedVec<T: ScalarInnerVecs> = T::InnerAlignedVec4;
}

#[doc(hidden)]
#[allow(private_bounds)]
pub trait VecAlignmentInnerVecs: Seal {
    type InnerVec<const N: usize, T: ScalarInnerVecs>: InnerConstruct
    where
        ScalarCount<N>: VecLenInnerVec;
}
impl Seal for VecPacked {}
impl VecAlignmentInnerVecs for VecPacked {
    type InnerVec<const N: usize, T: ScalarInnerVecs> = [T; N] where ScalarCount<N>: VecLenInnerVec;
}
impl Seal for VecAligned {}
impl VecAlignmentInnerVecs for VecAligned {
    type InnerVec<const N: usize, T: ScalarInnerVecs> =
        <ScalarCount<N> as VecLenInnerVec>::InnerAlignedVec<T> where ScalarCount<N>: VecLenInnerVec;
}

trait Seal: Sized {}
