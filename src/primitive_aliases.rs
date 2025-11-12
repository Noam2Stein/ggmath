/// `f32` type aliases
pub mod f32 {
    use crate::Vector;

    /// A 2D vector of `f32` elements
    pub type Vec2f = Vector<2, f32>;

    /// A 3D vector of `f32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec3f = Vector<3, f32>;

    /// A 4D vector of `f32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec4f = Vector<4, f32>;
}

/// `f64` type aliases
pub mod f64 {
    use crate::Vector;

    /// A 2D vector of `f64` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec2d = Vector<2, f64>;

    /// A 3D vector of `f64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec3d = Vector<3, f64>;

    /// A 4D vector of `f64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec4d = Vector<4, f64>;
}

/// `i8` type aliases
pub mod i8 {
    use crate::Vector;

    /// A 2D vector of `i8` elements
    pub type Vec2i8 = Vector<2, i8>;

    /// A 3D vector of `i8` elements
    pub type Vec3i8 = Vector<3, i8>;

    /// A 4D vector of `i8` elements
    pub type Vec4i8 = Vector<4, i8>;
}

/// `i16` type aliases
pub mod i16 {
    use crate::Vector;

    /// A 2D vector of `i16` elements
    pub type Vec2i16 = Vector<2, i16>;

    /// A 3D vector of `i16` elements
    pub type Vec3i16 = Vector<3, i16>;

    /// A 4D vector of `i16` elements
    pub type Vec4i16 = Vector<4, i16>;
}

/// `i32` type aliases
pub mod i32 {
    use crate::Vector;

    /// A 2D vector of `i32` elements
    pub type Vec2i = Vector<2, i32>;

    /// A 3D vector of `i32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec3i = Vector<3, i32>;

    /// A 4D vector of `i32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec4i = Vector<4, i32>;
}

/// `i64` type aliases
pub mod i64 {
    use crate::Vector;

    /// A 2D vector of `i64` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec2i64 = Vector<2, i64>;

    /// A 3D vector of `i64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec3i64 = Vector<3, i64>;

    /// A 4D vector of `i64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec4i64 = Vector<4, i64>;
}

/// `i128` type aliases
pub mod i128 {
    use crate::Vector;

    /// A 2D vector of `i128` elements
    pub type Vec2i128 = Vector<2, i128>;

    /// A 3D vector of `i128` elements
    pub type Vec3i128 = Vector<3, i128>;

    /// A 4D vector of `i128` elements
    pub type Vec4i128 = Vector<4, i128>;
}

/// `isize` type aliases
pub mod isize {
    use crate::Vector;

    /// A 2D vector of `isize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec2isize = Vector<2, isize>;

    /// A 3D vector of `isize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec3isize = Vector<3, isize>;

    /// A 4D vector of `isize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec4isize = Vector<4, isize>;
}

/// `u8` type aliases
pub mod u8 {
    use crate::Vector;

    /// A 2D vector of `u8` elements
    pub type Vec2u8 = Vector<2, u8>;

    /// A 3D vector of `u8` elements
    pub type Vec3u8 = Vector<3, u8>;

    /// A 4D vector of `u8` elements
    pub type Vec4u8 = Vector<4, u8>;
}

/// `u16` type aliases
pub mod u16 {
    use crate::Vector;

    /// A 2D vector of `u16` elements
    pub type Vec2u16 = Vector<2, u16>;

    /// A 3D vector of `u16` elements
    pub type Vec3u16 = Vector<3, u16>;

    /// A 4D vector of `u16` elements
    pub type Vec4u16 = Vector<4, u16>;
}

/// `u32` type aliases
pub mod u32 {
    use crate::Vector;

    /// A 2D vector of `u32` elements
    pub type Vec2u = Vector<2, u32>;

    /// A 3D vector of `u32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec3u = Vector<3, u32>;

    /// A 4D vector of `u32` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec4u = Vector<4, u32>;
}

/// `u64` type aliases
pub mod u64 {
    use crate::Vector;

    /// A 2D vector of `u64` elements.
    ///
    /// This type is 16-byte aligned on appropriate targets.
    pub type Vec2u64 = Vector<2, u64>;

    /// A 3D vector of `u64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec3u64 = Vector<3, u64>;

    /// A 4D vector of `u64` elements.
    ///
    /// This type is 32-byte aligned on appropriate targets.
    pub type Vec4u64 = Vector<4, u64>;
}

/// `u128` type aliases
pub mod u128 {
    use crate::Vector;

    /// A 2D vector of `u128` elements
    pub type Vec2u128 = Vector<2, u128>;

    /// A 3D vector of `u128` elements
    pub type Vec3u128 = Vector<3, u128>;

    /// A 4D vector of `u128` elements
    pub type Vec4u128 = Vector<4, u128>;
}

/// `usize` type aliases
pub mod usize {
    use crate::Vector;

    /// A 2D vector of `usize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec2usize = Vector<2, usize>;

    /// A 3D vector of `usize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec3usize = Vector<3, usize>;

    /// A 4D vector of `usize` elements.
    ///
    /// This type is SIMD-aligned on appropriate targets.
    pub type Vec4usize = Vector<4, usize>;
}

/// `bool` type aliases
pub mod bool {
    use crate::Vector;

    /// A 2D vector of `bool` elements
    pub type Vec2b = Vector<2, bool>;

    /// A 3D vector of `bool` elements
    pub type Vec3b = Vector<3, bool>;

    /// A 4D vector of `bool` elements
    pub type Vec4b = Vector<4, bool>;
}
