use super::*;

/// The quaternion type.
///
/// In most cases you can use this type's type aliases instead.
/// See in [`crate::quaternion`].
///
/// This type is generic over scalar type and alignment,
/// which follows the generics of [`Vector`].
#[derive_where(Clone, Copy)]
#[derive_where(Debug, Eq, Hash; T)]
pub struct Quaternion<T: Scalar, A: VecAlignment> {
    pub(super) inner: Vector<4, T, A>,
}

/// Type alias to [`Quaternion<T, VecAligned>`].
pub type Quat<T> = Quaternion<T, VecAligned>;

/// Type alias to [`Quaternion<T, VecPacked>`].
pub type QuatP<T> = Quaternion<T, VecPacked>;
