use derive_where::derive_where;

use super::*;

/// Axis-aligned bounding box.
///
/// In most cases, you can use the type aliases of this type (`Rect`, `Aabb3`, etc.).
/// See [`crate::aabb`].
///
/// This type is generic over the number of dimensions,
/// the scalar type, the vector alignment, which all follow `Vector`'s generics,
/// and the representation of the bounding box.
///
/// Aabbs can be represented in different ways:
/// - by their corners,
/// - by their center and size,
/// - any combination of these.
///
/// The type parameter `R` is the representation of the bounding box.
/// - `AabbCornered` represents the bounding box by its minimum corner and size.
/// - `AabbCentered` represents the bounding box by its center and extents.
/// - `AabbMinMaxed` represents the bounding box by its minimum and maximum corners.
///
/// ### Impl Pattern
///
/// This is how you create an impl block that applies to all aabbs:
///
/// ```rust
/// impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
/// where
///     Usize<N>: VecLen,
/// {
///     // ...
/// }
/// ```
#[derive_where(Clone, Copy)]
pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
where
    Usize<N>: VecLen,
{
    pub(super) inner: R::InnerAabb<N, T, A>,
}

// 2D

/// Type aliases to `Aabb<2, T, VecAligned, AabbCornered>`.
///
/// This means an aligned rectangle type represented by its minimum corner and size.
pub type Rect<T> = Aabb<2, T, VecAligned, AabbCornered>;

/// Type aliases to `Aabb<2, T, VecPacked, AabbCornered>`.
///
/// This means an unaligned rectangle type represented by its minimum corner and size.
pub type RectP<T> = Aabb<2, T, VecPacked, AabbCornered>;

/// Type aliases to `Aabb<2, T, VecAligned, AabbCentered>`.
///
/// This means an aligned rectangle type represented by its center and extents.
pub type RectC<T> = Aabb<2, T, VecAligned, AabbCentered>;

/// Type aliases to `Aabb<2, T, VecPacked, AabbCentered>`.
///
/// This means an unaligned rectangle type represented by its center and extents.
pub type RectCP<T> = Aabb<2, T, VecPacked, AabbCentered>;

/// Type aliases to `Aabb<2, T, VecAligned, AabbMinMaxed>`.
///
/// This means an aligned rectangle type represented by its minimum and maximum corners.
pub type RectM<T> = Aabb<2, T, VecAligned, AabbMinMaxed>;

/// Type aliases to `Aabb<2, T, VecPacked, AabbMinMaxed>`.
///
/// This means an unaligned rectangle type represented by its minimum and maximum corners.
pub type RectMP<T> = Aabb<2, T, VecPacked, AabbMinMaxed>;

// 3D

/// Type aliases to `Aabb<3, T, VecAligned, AabbCornered>`.
///
/// This means an aligned box type represented by its minimum corner and size.
pub type Aabb3<T> = Aabb<3, T, VecAligned, AabbCornered>;

/// Type aliases to `Aabb<3, T, VecPacked, AabbCornered>`.
///
/// This means an unaligned box type represented by its minimum corner and size.
pub type Aabb3P<T> = Aabb<3, T, VecPacked, AabbCornered>;

/// Type aliases to `Aabb<3, T, VecAligned, AabbCentered>`.
///
/// This means an aligned box type represented by its center and extents.
pub type Aabb3C<T> = Aabb<3, T, VecAligned, AabbCentered>;

/// Type aliases to `Aabb<3, T, VecPacked, AabbCentered>`.
///
/// This means an unaligned box type represented by its center and extents.
pub type Aabb3CP<T> = Aabb<3, T, VecPacked, AabbCentered>;

/// Type aliases to `Aabb<3, T, VecAligned, AabbMinMaxed>`.
///
/// This means an aligned box type represented by its minimum and maximum corners.
pub type Aabb3M<T> = Aabb<3, T, VecAligned, AabbMinMaxed>;

/// Type aliases to `Aabb<3, T, VecPacked, AabbMinMaxed>`.
///
/// This means an unaligned box type represented by its minimum and maximum corners.
pub type Aabb3MP<T> = Aabb<3, T, VecPacked, AabbMinMaxed>;

// 4D

/// Type aliases to `Aabb<4, T, VecAligned, AabbCornered>`.
///
/// This means an aligned 4D box type represented by its minimum corner and size.
pub type Aabb4<T> = Aabb<4, T, VecAligned, AabbCornered>;

/// Type aliases to `Aabb<4, T, VecPacked, AabbCornered>`.
///
/// This means an unaligned 4D box type represented by its minimum corner and size.
pub type Aabb4P<T> = Aabb<4, T, VecPacked, AabbCornered>;

/// Type aliases to `Aabb<4, T, VecAligned, AabbCentered>`.
///
/// This means an aligned 4D box type represented by its center and extents.
pub type Aabb4C<T> = Aabb<4, T, VecAligned, AabbCentered>;

/// Type aliases to `Aabb<4, T, VecPacked, AabbCentered>`.
///
/// This means an unaligned 4D box type represented by its center and extents.
pub type Aabb4CP<T> = Aabb<4, T, VecPacked, AabbCentered>;

/// Type aliases to `Aabb<4, T, VecAligned, AabbMinMaxed>`.
///
/// This means an aligned 4D box type represented by its minimum and maximum corners.
pub type Aabb4M<T> = Aabb<4, T, VecAligned, AabbMinMaxed>;

/// Type aliases to `Aabb<4, T, VecPacked, AabbMinMaxed>`.
///
/// This means an unaligned 4D box type represented by its minimum and maximum corners.
pub type Aabb4MP<T> = Aabb<4, T, VecPacked, AabbMinMaxed>;
