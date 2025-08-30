use crate::vector::{VecAligned, VecPacked, Vector};

/// Type alias for `Vector<2, T, VecAligned>`.
/// This is considered the default vec2 type.
///
/// This type is SIMD aligned.
/// For a non SIMD aligned vec2, use `Vec2P<T>`.
pub type Vec2<T> = Vector<2, T, VecAligned>;
/// Type alias for `Vector<3, T, VecAligned>`.
/// This is considered the default vec3 type.
///
/// This type is SIMD aligned.
/// For a non SIMD aligned vec3, use `Vec3P<T>`.
pub type Vec3<T> = Vector<3, T, VecAligned>;
/// Type alias for `Vector<4, T, VecAligned>`.
/// This is considered the default vec4 type.
///
/// This type is SIMD aligned.
/// For a non SIMD aligned vec4, use `Vec4P<T>`.
pub type Vec4<T> = Vector<4, T, VecAligned>;

/// Type alias for `Vector<2, T, VecPacked>`.
/// The `P` means the vector is not SIMD aligned, unlike `Vec2<T>`.
///
/// This should be used in memory critical situations,
/// while `Vec2<T>` should be used when operation performance is more important than memory usage,
/// which is most of the time.
pub type Vec2P<T> = Vector<2, T, VecPacked>;
/// Type alias for `Vector<3, T, VecPacked>`.
/// The `P` means the vector is not SIMD aligned, unlike `Vec3<T>`.
///
/// This should be used in memory critical situations,
/// while `Vec3<T>` should be used when operation performance is more important than memory usage,
/// which is most of the time.
pub type Vec3P<T> = Vector<3, T, VecPacked>;
/// Type alias for `Vector<4, T, VecPacked>`.
/// The `P` means the vector is not SIMD aligned, unlike `Vec4<T>`.
///
/// This should be used in memory critical situations,
/// while `Vec4<T>` should be used when operation performance is more important than memory usage,
/// which is most of the time.
pub type Vec4P<T> = Vector<4, T, VecPacked>;

/// Macro that generates vector type aliases for a specific scalar type.
///
/// Syntax: `vector_aliases!(<visibility> <prefix> => <scalar>)`
///
/// # Examples
///
/// ```
/// vector_aliases!(pub F => f32);
/// ```
/// Generates:
/// ```
/// pub type FVec2 = Vec2<f32>;
/// pub type FVec3 = Vec3<f32>;
/// pub type FVec4 = Vec4<f32>;
/// pub type FVec2P = Vec2P<f32>;
/// pub type FVec3P = Vec3P<f32>;
/// pub type FVec4P = Vec4P<f32>;
/// ```
#[macro_export]
macro_rules! vector_aliases {
    (pub($($vis:tt)*) $prefix:ident => $t:ty) => {
        $crate::vector_aliases!(@(pub $($vis)*) $prefix => $t);
    };
    (pub $prefix:ident => $t:ty) => {
        $crate::vector_aliases!(@(pub) $prefix => $t);
    };
    ($prefix:ident => $t:ty) => {
        $crate::vector_aliases!(@() $prefix => $t);
    };

    (@($($vis:tt)*) $prefix:ident => $t:ty) => {
        $crate::_hidden_::paste::paste! {
            #[doc = "Type alias to `Vector<2, " $t ", VecAligned>`.\n\nThis type is SIMD aligned.\nFor a non SIMD aligned vec2, use `" $prefix "Vec2P`."]
            $($vis)* type [<$prefix Vec2>] = $crate::aliases::Vec2<$t>;
            #[doc = "Type alias to `Vector<3, " $t ", VecAligned>`.\n\nThis type is SIMD aligned.\nFor a non SIMD aligned vec3, use `" $prefix "Vec3P`."]
            $($vis)* type [<$prefix Vec3>] = $crate::aliases::Vec3<$t>;
            #[doc = "Type alias to `Vector<4, " $t ", VecAligned>`.\n\nThis type is SIMD aligned.\nFor a non SIMD aligned vec4, use `" $prefix "Vec4P`."]
            $($vis)* type [<$prefix Vec4>] = $crate::aliases::Vec4<$t>;

            #[doc = "Type alias to `Vector<2, " $t ", VecPacked>`.\n\nThe `P` means the vector is not SIMD aligned, unlike `" $prefix "Vec2`.\n\nThis should be used in memory critical situations,\nwhile `" $prefix "Vec2` should be used when operation performance is more important than memory usage,\nwhich is most of the time."]
            $($vis)* type [<$prefix Vec2P>] = $crate::aliases::Vec2P<$t>;
            #[doc = "Type alias to `Vector<3, " $t ", VecPacked>`.\n\nThe `P` means the vector is not SIMD aligned, unlike `" $prefix "Vec3`.\n\nThis should be used in memory critical situations,\nwhile `" $prefix "Vec3` should be used when operation performance is more important than memory usage,\nwhich is most of the time."]
            $($vis)* type [<$prefix Vec3P>] = $crate::aliases::Vec3P<$t>;
            #[doc = "Type alias to `Vector<4, " $t ", VecPacked>`.\n\nThe `P` means the vector is not SIMD aligned, unlike `" $prefix "Vec4`.\n\nThis should be used in memory critical situations,\nwhile `" $prefix "Vec4` should be used when operation performance is more important than memory usage,\nwhich is most of the time."]
            $($vis)* type [<$prefix Vec4P>] = $crate::aliases::Vec4P<$t>;
        }
    };
}
