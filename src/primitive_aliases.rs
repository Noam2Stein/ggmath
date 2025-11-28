/// [`f32`] type aliases.
pub mod f32 {
    #[cfg(feature = "affine")]
    use crate::Affine;
    #[cfg(feature = "matrix")]
    use crate::Matrix;
    #[cfg(feature = "quaternion")]
    use crate::Quaternion;
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`f32`] elements.
    pub type Vec2f = Vector<2, f32, Unaligned>;

    /// A 3D vector of [`f32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3fA`] instead.
    pub type Vec3f = Vector<3, f32, Unaligned>;

    /// A 4D vector of [`f32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4fA`] instead.
    pub type Vec4f = Vector<4, f32, Unaligned>;

    /// A 3D vector of [`f32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse`.
    pub type Vec3fA = Vector<3, f32, Aligned>;

    /// A 4D vector of [`f32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse`.
    pub type Vec4fA = Vector<4, f32, Aligned>;

    /// A 2x2 column major matrix of [`f32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat2fA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat2f = Matrix<2, f32, Unaligned>;

    /// A 3x3 column major matrix of [`f32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat3fA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat3f = Matrix<3, f32, Unaligned>;

    /// A 4x4 column major matrix of [`f32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat4fA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat4f = Matrix<4, f32, Unaligned>;

    /// A 2x2 column major matrix of [`f32`] elements.
    ///
    /// This type is internally stored as a [`Vec4fA`].
    #[cfg(feature = "matrix")]
    pub type Mat2fA = Matrix<2, f32, Aligned>;

    /// A 3x3 column major matrix of [`f32`] elements.
    ///
    /// This type is internally stored as a `[Vec3fA; 3]`.
    #[cfg(feature = "matrix")]
    pub type Mat3fA = Matrix<3, f32, Aligned>;

    /// A 4x4 column major matrix of [`f32`] elements.
    ///
    /// This type is internally stored as a `[Vec4fA; 4]`.
    #[cfg(feature = "matrix")]
    pub type Mat4fA = Matrix<4, f32, Aligned>;

    /// A quaternion stored using [`f32`]s.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`QuatfA`] instead.
    #[cfg(feature = "quaternion")]
    pub type Quatf = Quaternion<f32, Unaligned>;

    /// A quaternion stored using [`f32`]s.
    ///
    /// This type is internally stored as a [`Vec4fA`].
    #[cfg(feature = "quaternion")]
    pub type QuatfA = Quaternion<f32, Aligned>;

    /// An [`f32`] 2D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Affine2fA`] instead.
    #[cfg(feature = "affine")]
    pub type Affine2f = Affine<2, f32, Unaligned>;

    /// An [`f32`] 3D affine transform, which can represent translation, rotation,
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Affine3fA`] instead.
    #[cfg(feature = "affine")]
    pub type Affine3f = Affine<3, f32, Unaligned>;

    /// An [`f32`] 2D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is internally stored with a [`Vec4fA`] for its matrix.
    #[cfg(feature = "affine")]
    pub type Affine2fA = Affine<2, f32, Aligned>;

    /// An [`f32`] 3D affine transform, which can represent translation, rotation,
    ///
    /// This type is internally stored as `[Vec3fA; 4]`.
    #[cfg(feature = "affine")]
    pub type Affine3fA = Affine<3, f32, Aligned>;
}

/// [`f64`] type aliases.
pub mod f64 {
    #[cfg(feature = "affine")]
    use crate::Affine;
    #[cfg(feature = "matrix")]
    use crate::Matrix;
    #[cfg(feature = "quaternion")]
    use crate::Quaternion;
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec2dA`] instead.
    pub type Vec2d = Vector<2, f64, Unaligned>;

    /// A 3D vector of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3dA`] instead.
    pub type Vec3d = Vector<3, f64, Unaligned>;

    /// A 4D vector of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4dA`] instead.
    pub type Vec4d = Vector<4, f64, Unaligned>;

    /// A 2D vector of [`f64`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec2dA = Vector<2, f64, Aligned>;

    /// A 3D vector of [`f64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx`.
    pub type Vec3dA = Vector<3, f64, Aligned>;

    /// A 4D vector of [`f64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx`.
    pub type Vec4dA = Vector<4, f64, Aligned>;

    /// A 2D column major matrix of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat2dA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat2d = Matrix<2, f64, Unaligned>;

    /// A 3D column major matrix of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat3dA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat3d = Matrix<3, f64, Unaligned>;

    /// A 4D column major matrix of [`f64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Mat4dA`] instead.
    #[cfg(feature = "matrix")]
    pub type Mat4d = Matrix<4, f64, Unaligned>;

    /// A 2D column major matrix of [`f64`] elements.
    ///
    /// This type is internally stored as [`Vec4dA`].
    #[cfg(feature = "matrix")]
    pub type Mat2dA = Matrix<2, f64, Aligned>;

    /// A 3D column major matrix of [`f64`] elements.
    ///
    /// This type is internally stored as `[Vec3dA; 3]`.
    #[cfg(feature = "matrix")]
    pub type Mat3dA = Matrix<3, f64, Aligned>;

    /// A 4D column major matrix of [`f64`] elements.
    ///
    /// This type is internally stored as `[Vec4dA; 4]`.
    #[cfg(feature = "matrix")]
    pub type Mat4dA = Matrix<4, f64, Aligned>;

    /// A quaternion stored using [`f64`]s.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`QuatdA`] instead.
    #[cfg(feature = "quaternion")]
    pub type Quatd = Quaternion<f64, Unaligned>;

    /// A quaternion stored using [`f64`]s.
    ///
    /// This type is internally stored as a [`Vec4dA`].
    #[cfg(feature = "quaternion")]
    pub type QuatdA = Quaternion<f64, Aligned>;

    /// An [`f64`] 2D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Affine2dA`] instead.
    #[cfg(feature = "affine")]
    pub type Affine2d = Affine<2, f64, Unaligned>;

    /// An [`f64`] 3D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Affine3dA`] instead.
    #[cfg(feature = "affine")]
    pub type Affine3d = Affine<3, f64, Unaligned>;

    /// An [`f64`] 2D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is internally stored with a [`Vec4dA`] for its matrix.
    #[cfg(feature = "affine")]
    pub type Affine2dA = Affine<2, f64, Aligned>;

    /// An [`f64`] 3D affine transform, which can represent translation, rotation,
    /// scaling, shear.
    ///
    /// This type is internally stored as `[Vec3dA; 4]`.
    #[cfg(feature = "affine")]
    pub type Affine3dA = Affine<3, f64, Aligned>;
}

/// [`i8`] type aliases.
pub mod i8 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`i8`] elements.
    pub type Vec2i8 = Vector<2, i8, Unaligned>;

    /// A 3D vector of [`i8`] elements.
    pub type Vec3i8 = Vector<3, i8, Unaligned>;

    /// A 4D vector of [`i8`] elements.
    pub type Vec4i8 = Vector<4, i8, Unaligned>;
}

/// [`i16`] type aliases.
pub mod i16 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`i16`] elements.
    pub type Vec2i16 = Vector<2, i16, Unaligned>;

    /// A 3D vector of [`i16`] elements.
    pub type Vec3i16 = Vector<3, i16, Unaligned>;

    /// A 4D vector of [`i16`] elements.
    pub type Vec4i16 = Vector<4, i16, Unaligned>;
}

/// [`i32`] type aliases.
pub mod i32 {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`i32`] elements.
    pub type Vec2i = Vector<2, i32, Unaligned>;

    /// A 3D vector of [`i32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3iA`] instead.
    pub type Vec3i = Vector<3, i32, Unaligned>;

    /// A 4D vector of [`i32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4iA`] instead.
    pub type Vec4i = Vector<4, i32, Unaligned>;

    /// A 3D vector of [`i32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec3iA = Vector<3, i32, Aligned>;

    /// A 4D vector of [`i32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec4iA = Vector<4, i32, Aligned>;
}

/// [`i64`] type aliases.
pub mod i64 {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`i64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec2i64A`] instead.
    pub type Vec2i64 = Vector<2, i64, Unaligned>;

    /// A 3D vector of [`i64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3i64A`] instead.
    pub type Vec3i64 = Vector<3, i64, Unaligned>;

    /// A 4D vector of [`i64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4i64A`] instead.
    pub type Vec4i64 = Vector<4, i64, Unaligned>;

    /// A 2D vector of [`i64`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec2i64A = Vector<2, i64, Aligned>;

    /// A 3D vector of [`i64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx2`.
    pub type Vec3i64A = Vector<3, i64, Aligned>;

    /// A 4D vector of [`i64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx2`.
    pub type Vec4i64A = Vector<4, i64, Aligned>;
}

/// [`i128`] type aliases.
pub mod i128 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`i128`] elements.
    pub type Vec2i128 = Vector<2, i128, Unaligned>;

    /// A 3D vector of [`i128`] elements.
    pub type Vec3i128 = Vector<3, i128, Unaligned>;

    /// A 4D vector of [`i128`] elements.
    pub type Vec4i128 = Vector<4, i128, Unaligned>;
}

/// [`isize`] type aliases.
pub mod isize {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`isize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec2isizeA`] instead.
    pub type Vec2isize = Vector<2, isize, Unaligned>;

    /// A 3D vector of [`isize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3isizeA`] instead.
    pub type Vec3isize = Vector<3, isize, Unaligned>;

    /// A 4D vector of [`isize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4isizeA`] instead.
    pub type Vec4isize = Vector<4, isize, Unaligned>;

    /// A 2D vector of [`isize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec2isizeA = Vector<2, isize, Aligned>;

    /// A 3D vector of [`isize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec3isizeA = Vector<3, isize, Aligned>;

    /// A 4D vector of [`isize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec4isizeA = Vector<4, isize, Aligned>;
}

/// [`u8`] type aliases.
pub mod u8 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`u8`] elements.
    pub type Vec2u8 = Vector<2, u8, Unaligned>;

    /// A 3D vector of [`u8`] elements.
    pub type Vec3u8 = Vector<3, u8, Unaligned>;

    /// A 4D vector of [`u8`] elements.
    pub type Vec4u8 = Vector<4, u8, Unaligned>;
}

/// [`u16`] type aliases.
pub mod u16 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`u16`] elements.
    pub type Vec2u16 = Vector<2, u16, Unaligned>;

    /// A 3D vector of [`u16`] elements.
    pub type Vec3u16 = Vector<3, u16, Unaligned>;

    /// A 4D vector of [`u16`] elements.
    pub type Vec4u16 = Vector<4, u16, Unaligned>;
}

/// [`u32`] type aliases.
pub mod u32 {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`u32`] elements.
    pub type Vec2u = Vector<2, u32, Unaligned>;

    /// A 3D vector of [`u32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3uA`] instead.
    pub type Vec3u = Vector<3, u32, Unaligned>;

    /// A 4D vector of [`u32`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4uA`] instead.
    pub type Vec4u = Vector<4, u32, Unaligned>;

    /// A 3D vector of [`u32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec3uA = Vector<3, u32, Aligned>;

    /// A 4D vector of [`u32`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec4uA = Vector<4, u32, Aligned>;
}

/// [`u64`] type aliases.
pub mod u64 {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`u64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec2u64A`] instead.
    pub type Vec2u64 = Vector<2, u64, Unaligned>;

    /// A 3D vector of [`u64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3u64A`] instead.
    pub type Vec3u64 = Vector<3, u64, Unaligned>;

    /// A 4D vector of [`u64`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4u64A`] instead.
    pub type Vec4u64 = Vector<4, u64, Unaligned>;

    /// A 2D vector of [`u64`] elements.
    ///
    /// This type is aligned to 16 bytes if the target platform supports `sse2`.
    pub type Vec2u64A = Vector<2, u64, Aligned>;

    /// A 3D vector of [`u64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx2`.
    pub type Vec3u64A = Vector<3, u64, Aligned>;

    /// A 4D vector of [`u64`] elements.
    ///
    /// This type is aligned to 32 bytes if the target platform supports `avx2`.
    pub type Vec4u64A = Vector<4, u64, Aligned>;
}

/// [`u128`] type aliases.
pub mod u128 {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`u128`] elements.
    pub type Vec2u128 = Vector<2, u128, Unaligned>;

    /// A 3D vector of [`u128`] elements.
    pub type Vec3u128 = Vector<3, u128, Unaligned>;

    /// A 4D vector of [`u128`] elements.
    pub type Vec4u128 = Vector<4, u128, Unaligned>;
}

/// [`usize`] type aliases.
pub mod usize {
    use crate::{Aligned, Unaligned, Vector};

    /// A 2D vector of [`usize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec2usizeA`] instead.
    pub type Vec2usize = Vector<2, usize, Unaligned>;

    /// A 3D vector of [`usize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec3usizeA`] instead.
    pub type Vec3usize = Vector<3, usize, Unaligned>;

    /// A 4D vector of [`usize`] elements.
    ///
    /// This type is *not* SIMD-aligned. For SIMD-alignment, use [`Vec4usizeA`] instead.
    pub type Vec4usize = Vector<4, usize, Unaligned>;

    /// A 2D vector of [`usize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec2usizeA = Vector<2, usize, Aligned>;

    /// A 3D vector of [`usize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec3usizeA = Vector<3, usize, Aligned>;

    /// A 4D vector of [`usize`] elements.
    ///
    /// This type may be SIMD-aligned based on the target platform.
    pub type Vec4usizeA = Vector<4, usize, Aligned>;
}

/// [`bool`] type aliases.
pub mod bool {
    use crate::{Unaligned, Vector};

    /// A 2D vector of [`bool`] elements.
    pub type Vec2b = Vector<2, bool, Unaligned>;

    /// A 3D vector of [`bool`] elements.
    pub type Vec3b = Vector<3, bool, Unaligned>;

    /// A 4D vector of [`bool`] elements.
    pub type Vec4b = Vector<4, bool, Unaligned>;
}
