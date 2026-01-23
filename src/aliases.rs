use crate::{Aligned, Unaligned, Vector};

/// A 2-dimensional vector.
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec2<T> = Vector<2, T, Aligned>;

/// A 3-dimensional vector.
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec3<T> = Vector<3, T, Aligned>;

/// A 4-dimensional vector.
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec4<T> = Vector<4, T, Aligned>;

/// A 2-dimensional vector.
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 2]`.
pub type Vec2U<T> = Vector<2, T, Unaligned>;

/// A 3-dimensional vector.
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 3]`.
pub type Vec3U<T> = Vector<3, T, Unaligned>;

/// A 4-dimensional vector.
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 4]`.
pub type Vec4U<T> = Vector<4, T, Unaligned>;
