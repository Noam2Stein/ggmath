use crate::vector::{Aligned, Unaligned, Vector};

////////////////////////////////////////////////////////////////////////////////
// `f32`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`f32`] elements.
pub type Vec2 = Vector<2, f32, Unaligned>;

/// A 3D vector of [`f32`] elements.
pub type Vec3 = Vector<3, f32, Unaligned>;

/// A 4D vector of [`f32`] elements.
pub type Vec4 = Vector<4, f32, Unaligned>;

/// A 3D vector of [`f32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type Vec3A = Vector<3, f32, Aligned>;

/// A 4D vector of [`f32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type Vec4A = Vector<4, f32, Aligned>;

////////////////////////////////////////////////////////////////////////////////
// `f64`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`f64`] elements.
pub type DVec2 = Vector<2, f64, Unaligned>;

/// A 3D vector of [`f64`] elements.
pub type DVec3 = Vector<3, f64, Unaligned>;

/// A 4D vector of [`f64`] elements.
pub type DVec4 = Vector<4, f64, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `i8`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`i8`] elements.
pub type I8Vec2 = Vector<2, i8, Unaligned>;

/// A 3D vector of [`i8`] elements.
pub type I8Vec3 = Vector<3, i8, Unaligned>;

/// A 4D vector of [`i8`] elements.
pub type I8Vec4 = Vector<4, i8, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `i16`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`i16`] elements.
pub type I16Vec2 = Vector<2, i16, Unaligned>;

/// A 3D vector of [`i16`] elements.
pub type I16Vec3 = Vector<3, i16, Unaligned>;

/// A 4D vector of [`i16`] elements.
pub type I16Vec4 = Vector<4, i16, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `i32`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`i32`] elements.
pub type IVec2 = Vector<2, i32, Unaligned>;

/// A 3D vector of [`i32`] elements.
pub type IVec3 = Vector<3, i32, Unaligned>;

/// A 4D vector of [`i32`] elements.
pub type IVec4 = Vector<4, i32, Unaligned>;

/// A 3D vector of [`i32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type IVec3A = Vector<3, i32, Aligned>;

/// A 4D vector of [`i32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type IVec4A = Vector<4, i32, Aligned>;

////////////////////////////////////////////////////////////////////////////////
// `i64`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`i64`] elements.
pub type I64Vec2 = Vector<2, i64, Unaligned>;

/// A 3D vector of [`i64`] elements.
pub type I64Vec3 = Vector<3, i64, Unaligned>;

/// A 4D vector of [`i64`] elements.
pub type I64Vec4 = Vector<4, i64, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `isize`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`isize`] elements.
pub type ISizeVec2 = Vector<2, isize, Unaligned>;

/// A 3D vector of [`isize`] elements.
pub type ISizeVec3 = Vector<3, isize, Unaligned>;

/// A 4D vector of [`isize`] elements.
pub type ISizeVec4 = Vector<4, isize, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `u8`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`u8`] elements.
pub type U8Vec2 = Vector<2, u8, Unaligned>;

/// A 3D vector of [`u8`] elements.
pub type U8Vec3 = Vector<3, u8, Unaligned>;

/// A 4D vector of [`u8`] elements.
pub type U8Vec4 = Vector<4, u8, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `u16`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`u16`] elements.
pub type U16Vec2 = Vector<2, u16, Unaligned>;

/// A 3D vector of [`u16`] elements.
pub type U16Vec3 = Vector<3, u16, Unaligned>;

/// A 4D vector of [`u16`] elements.
pub type U16Vec4 = Vector<4, u16, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `u32`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`u32`] elements.
pub type UVec2 = Vector<2, u32, Unaligned>;

/// A 3D vector of [`u32`] elements.
pub type UVec3 = Vector<3, u32, Unaligned>;

/// A 4D vector of [`u32`] elements.
pub type UVec4 = Vector<4, u32, Unaligned>;

/// A 3D vector of [`u32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type UVec3A = Vector<3, u32, Aligned>;

/// A 4D vector of [`u32`] elements.
///
/// This type is 16 byte aligned on most platforms.
pub type UVec4A = Vector<4, u32, Aligned>;

////////////////////////////////////////////////////////////////////////////////
// `u64`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`u64`] elements.
pub type U64Vec2 = Vector<2, u64, Unaligned>;

/// A 3D vector of [`u64`] elements.
pub type U64Vec3 = Vector<3, u64, Unaligned>;

/// A 4D vector of [`u64`] elements.
pub type U64Vec4 = Vector<4, u64, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `usize`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`usize`] elements.
pub type USizeVec2 = Vector<2, usize, Unaligned>;

/// A 3D vector of [`usize`] elements.
pub type USizeVec3 = Vector<3, usize, Unaligned>;

/// A 4D vector of [`usize`] elements.
pub type USizeVec4 = Vector<4, usize, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// `bool`
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of [`bool`] elements.
pub type BVec2 = Vector<2, bool, Unaligned>;

/// A 3D vector of [`bool`] elements.
pub type BVec3 = Vector<3, bool, Unaligned>;

/// A 4D vector of [`bool`] elements.
pub type BVec4 = Vector<4, bool, Unaligned>;

////////////////////////////////////////////////////////////////////////////////
// Special Types
////////////////////////////////////////////////////////////////////////////////

/// This module contains type aliases that are not useful for the average user.
///
/// This includes aligned variants of 64-bit types, which are only different
/// from their unaligned counterparts on platforms with 256-bit SIMD. This also
/// includes 128-bit types which are rarely needed.
pub mod rare {
    use crate::vector::{Aligned, Unaligned, Vector};

    ////////////////////////////////////////////////////////////////////////////////
    // `f64`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 3D vector of [`f64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type DVec3A = Vector<3, f64, Aligned>;

    /// A 4D vector of [`f64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type DVec4A = Vector<4, f64, Aligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `i64`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 3D vector of [`i64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type I64Vec3A = Vector<3, i64, Aligned>;

    /// A 4D vector of [`i64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type I64Vec4A = Vector<4, i64, Aligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `i128`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 2D vector of [`i128`] elements.
    pub type I128Vec2 = Vector<2, i128, Unaligned>;

    /// A 3D vector of [`i128`] elements.
    pub type I128Vec3 = Vector<3, i128, Unaligned>;

    /// A 4D vector of [`i128`] elements.
    pub type I128Vec4 = Vector<4, i128, Unaligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `isize`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 3D vector of [`isize`] elements.
    ///
    /// On 64-bit platforms, this type matches [`I64Vec3A`].
    /// On 32-bit platforms, this type matches [`IVec3A`](crate::IVec3A).
    pub type ISizeVec3A = Vector<3, isize, Aligned>;

    /// A 4D vector of [`isize`] elements.
    ///
    /// On 64-bit platforms, this type matches [`I64Vec4A`].
    /// On 32-bit platforms, this type matches [`IVec4A`](crate::IVec4A).
    pub type ISizeVec4A = Vector<4, isize, Aligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `u64`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 3D vector of [`u64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type U64Vec3A = Vector<3, u64, Aligned>;

    /// A 4D vector of [`u64`] elements.
    ///
    /// This type is 32 byte aligned on platforms with 256-bit SIMD.
    pub type U64Vec4A = Vector<4, u64, Aligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `u128`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 2D vector of [`u128`] elements.
    pub type U128Vec2 = Vector<2, u128, Unaligned>;

    /// A 3D vector of [`u128`] elements.
    pub type U128Vec3 = Vector<3, u128, Unaligned>;

    /// A 4D vector of [`u128`] elements.
    pub type U128Vec4 = Vector<4, u128, Unaligned>;

    ////////////////////////////////////////////////////////////////////////////////
    // `usize`
    ////////////////////////////////////////////////////////////////////////////////

    /// A 3D vector of [`usize`] elements.
    ///
    /// On 64-bit platforms, this type matches [`U64Vec3A`].
    /// On 32-bit platforms, this type matches [`UVec3A`](crate::UVec3A).
    pub type USizeVec3A = Vector<3, usize, Aligned>;

    /// A 4D vector of [`usize`] elements.
    ///
    /// On 64-bit platforms, this type matches [`U64Vec4A`].
    /// On 32-bit platforms, this type matches [`UVec4A`](crate::UVec4A).
    pub type USizeVec4A = Vector<4, usize, Aligned>;
}
