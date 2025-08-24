use std::mem::offset_of;

use derive_where::derive_where;

use super::*;

/// The vector type.
/// Statically-sized vector.
///
/// In most cases you can use this type's type aliases instead.
/// See in [`crate::vector`].
///
/// This type is generic over length, which can be either 2, 3 or 4.
///
/// This type is also generic over scalar type, which can be any type that implements [`Scalar`].
///
/// Finally,
/// this type is generic over whether its aligned for SIMD, or unaligned to save space.
/// See [`VecAlignment`] for more information.
#[derive_where(Clone, Copy)]
#[derive_where(Eq, Hash; T)]
pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub(super) array: [T; N],

    #[derive_where(skip)]
    pub(super) _align: A::Alignment<N, T>,
}

/// type alias to [`Vector<2, T, VecAligned>`].
pub type Vec2<T> = Vector<2, T, VecAligned>;

/// type alias to [`Vector<3, T, VecAligned>`].
pub type Vec3<T> = Vector<3, T, VecAligned>;

/// type alias to [`Vector<4, T, VecAligned>`].
pub type Vec4<T> = Vector<4, T, VecAligned>;

/// type alias to [`Vector<2, T, VecPacked>`].
/// See [`VecAlignment`]
pub type Vec2P<T> = Vector<2, T, VecPacked>;

/// type alias to [`Vector<3, T, VecPacked>`].
/// See [`VecAlignment`]
pub type Vec3P<T> = Vector<3, T, VecPacked>;

/// type alias to [`Vector<4, T, VecPacked>`].
/// See [`VecAlignment`]
pub type Vec4P<T> = Vector<4, T, VecPacked>;

// verify size
const _: () = assert!(size_of::<Vec2<f32>>() >= 8);
const _: () = assert!(size_of::<Vec3<f32>>() >= 12);
const _: () = assert!(size_of::<Vec4<f32>>() >= 16);
const _: () = assert!(size_of::<Vec2P<f32>>() == 8);
const _: () = assert!(size_of::<Vec3P<f32>>() == 12);
const _: () = assert!(size_of::<Vec4P<f32>>() == 16);

const _: () = assert!(size_of::<Vec2<bool>>() >= 2);
const _: () = assert!(size_of::<Vec3<bool>>() >= 3);
const _: () = assert!(size_of::<Vec4<bool>>() >= 4);
const _: () = assert!(size_of::<Vec2P<bool>>() == 2);
const _: () = assert!(size_of::<Vec3P<bool>>() == 3);
const _: () = assert!(size_of::<Vec4P<bool>>() == 4);

// verify alignment
const _: () = assert!(align_of::<Vec2<f32>>() >= align_of::<f32>());
const _: () = assert!(align_of::<Vec3<f32>>() >= align_of::<f32>());
const _: () = assert!(align_of::<Vec4<f32>>() >= align_of::<f32>());
const _: () = assert!(align_of::<Vec2P<f32>>() == align_of::<f32>());
const _: () = assert!(align_of::<Vec3P<f32>>() == align_of::<f32>());
const _: () = assert!(align_of::<Vec4P<f32>>() == align_of::<f32>());

const _: () = assert!(align_of::<Vec2<bool>>() >= align_of::<bool>());
const _: () = assert!(align_of::<Vec3<bool>>() >= align_of::<bool>());
const _: () = assert!(align_of::<Vec4<bool>>() >= align_of::<bool>());
const _: () = assert!(align_of::<Vec2P<bool>>() == align_of::<bool>());
const _: () = assert!(align_of::<Vec3P<bool>>() == align_of::<bool>());
const _: () = assert!(align_of::<Vec4P<bool>>() == align_of::<bool>());

// verify offset
const _: () = assert!(offset_of!(Vec2<f32>, array) == 0);
const _: () = assert!(offset_of!(Vec3<f32>, array) == 0);
const _: () = assert!(offset_of!(Vec4<f32>, array) == 0);
const _: () = assert!(offset_of!(Vec2P<f32>, array) == 0);
const _: () = assert!(offset_of!(Vec3P<f32>, array) == 0);
const _: () = assert!(offset_of!(Vec4P<f32>, array) == 0);

const _: () = assert!(offset_of!(Vec2<bool>, array) == 0);
const _: () = assert!(offset_of!(Vec3<bool>, array) == 0);
const _: () = assert!(offset_of!(Vec4<bool>, array) == 0);
const _: () = assert!(offset_of!(Vec2P<bool>, array) == 0);
const _: () = assert!(offset_of!(Vec3P<bool>, array) == 0);
const _: () = assert!(offset_of!(Vec4P<bool>, array) == 0);
