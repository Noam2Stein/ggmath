//! Quaternion type,

use derive_where::derive_where;

use super::{scalar::*, vector::*};

mod conversion;
mod new;
mod ops;

/// The quaternion type.
///
/// In most cases you can use this type's type aliases instead.
/// See in [`crate::quaternion`].
///
/// This type is generic over scalar type and alignment,
/// which follows the generics of [`Vector`].
///
/// ### Impl Pattern
///
/// This is how you make an impl block that applies to all quaternions:
///
/// ```
/// impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
/// }
/// ```
#[derive_where(Clone, Copy)]
#[derive_where(Debug, Eq, Hash; T)]
pub struct Quaternion<T: Scalar, A: VecAlignment> {
    inner: Vector<4, T, A>,
}

/// Type alias to [`Quaternion<T, VecAligned>`].
pub type Quat<T> = Quaternion<T, VecAligned>;

/// Type alias to [`Quaternion<T, VecPacked>`].
pub type QuatP<T> = Quaternion<T, VecPacked>;
