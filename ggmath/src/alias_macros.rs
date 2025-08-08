/// Expands to a declaration of type specific vector aliases.
///
/// Syntax:
/// `<vis> <prefix> => <type>`
///
/// Example:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// vector_aliases!(pub Big => BigInt);
/// ```
///
/// Expands to:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// pub type BigVec2 = Vector<2, BigInt, VecAligned>;
/// pub type BigVec3 = Vector<3, BigInt, VecAligned>;
/// pub type BigVec4 = Vector<4, BigInt, VecAligned>;
///
/// pub type BigVec2P = Vector<2, BigInt, VecPacked>;
/// pub type BigVec3P = Vector<3, BigInt, VecPacked>;
/// pub type BigVec4P = Vector<4, BigInt, VecPacked>;
/// ```
#[cfg(feature = "vector")]
#[macro_export]
macro_rules! vector_aliases {
    (pub($($vis:tt)*) $prefix:ident => $type:tt) => {
        vector_aliases! { @(pub($($vis)*)) $prefix => $type }
    };
    (pub $prefix:ident => $type:tt) => {
        vector_aliases! { @(pub) $prefix => $type }
    };
    ($prefix:ident => $type:tt) => {
        vector_aliases! { @() $prefix => $type }
    };

    (@($($vis:tt)*) $prefix:ident => $type:tt) => {
        $crate::repetitive! {
            #[doc = @str["Type alias for `Vector<2, " ~$type ", VecAligned>`"]]
            $($vis)* type @[~$prefix 'Vec2] = $crate::Vec2<$type>;

            #[doc = @str["Type alias for `Vector<3, " ~$type ", VecAligned>`"]]
            $($vis)* type @[~$prefix 'Vec3] = $crate::Vec3<$type>;

            #[doc = @str["Type alias for `Vector<4, " ~$type ", VecAligned>`"]]
            $($vis)* type @[~$prefix 'Vec4] = $crate::Vec4<$type>;

            #[doc = @str["Type alias for `Vector<2, " ~$type ", VecPacked>`"]]
            $($vis)* type @[~$prefix 'Vec2P] = $crate::Vec2P<$type>;

            #[doc = @str["Type alias for `Vector<3, " ~$type ", VecPacked>`"]]
            $($vis)* type @[~$prefix 'Vec3P] = $crate::Vec3P<$type>;

            #[doc = @str["Type alias for `Vector<4, " ~$type ", VecPacked>`"]]
            $($vis)* type @[~$prefix 'Vec4P] = $crate::Vec4P<$type>;
        }
    };
}

/// Expands to a declaration of type specific matrix aliases.
///
/// Syntax:
/// `<vis> <prefix> => <type>`
///
/// Example:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// matrix_aliases!(pub Big => BigInt);
/// ```
///
/// Expands to:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// pub type BigMat2C = Matrix<2, 2, BigInt, VecAligned, ColMajor>;
/// pub type BigMat2x3C = Matrix<2, 3, BigInt, VecAligned, ColMajor>;
/// pub type BigMat2x4C = Matrix<2, 4, BigInt, VecAligned, ColMajor>;
/// // ...
/// ```
#[cfg(feature = "matrix")]
#[macro_export]
macro_rules! matrix_aliases {
    (pub($($vis:tt)*) $prefix:ident => $type:tt) => {
        matrix_aliases! { @(pub($($vis)*)) $prefix => $type }
    };
    (pub $prefix:ident => $type:tt) => {
        matrix_aliases! { @(pub) $prefix => $type }
    };
    ($prefix:ident => $type:tt) => {
        matrix_aliases! { @() $prefix => $type }
    };

    (@($($vis:tt)*) $prefix:ident => $type:tt) => {
        $crate::repetitive! {
            // Column-Major, Aligned

            #[doc = @str["Type alias for `Matrix<2, 2, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2C] = $crate::Mat2C<$type>;
            #[doc = @str["Type alias for `Matrix<2, 3, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x3C] = $crate::Mat2x3C<$type>;
            #[doc = @str["Type alias for `Matrix<2, 4, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x4C] = $crate::Mat2x4C<$type>;

            #[doc = @str["Type alias for `Matrix<3, 2, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x2C] = $crate::Mat3x2C<$type>;
            #[doc = @str["Type alias for `Matrix<3, 3, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3C] = $crate::Mat3C<$type>;
            #[doc = @str["Type alias for `Matrix<3, 4, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x4C] = $crate::Mat3x4C<$type>;

            #[doc = @str["Type alias for `Matrix<4, 2, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x2C] = $crate::Mat4x2C<$type>;
            #[doc = @str["Type alias for `Matrix<4, 3, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x3C] = $crate::Mat4x3C<$type>;
            #[doc = @str["Type alias for `Matrix<4, 4, " ~$type ", VecAligned, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4C] = $crate::Mat4C<$type>;

            // Row-Major, Aligned

            #[doc = @str["Type alias for `Matrix<2, 2, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2R] = $crate::Mat2R<$type>;
            #[doc = @str["Type alias for `Matrix<2, 3, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x3R] = $crate::Mat2x3R<$type>;
            #[doc = @str["Type alias for `Matrix<2, 4, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x4R] = $crate::Mat2x4R<$type>;

            #[doc = @str["Type alias for `Matrix<3, 2, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x2R] = $crate::Mat3x2R<$type>;
            #[doc = @str["Type alias for `Matrix<3, 3, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3R] = $crate::Mat3R<$type>;
            #[doc = @str["Type alias for `Matrix<3, 4, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x4R] = $crate::Mat3x4R<$type>;

            #[doc = @str["Type alias for `Matrix<4, 2, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x2R] = $crate::Mat4x2R<$type>;
            #[doc = @str["Type alias for `Matrix<4, 3, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x3R] = $crate::Mat4x3R<$type>;
            #[doc = @str["Type alias for `Matrix<4, 4, " ~$type ", VecAligned, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4R] = $crate::Mat4R<$type>;

            // Column-Major, Packed

            #[doc = @str["Type alias for `Matrix<2, 2, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2CP] = $crate::Mat2CP<$type>;
            #[doc = @str["Type alias for `Matrix<2, 3, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x3CP] = $crate::Mat2x3CP<$type>;
            #[doc = @str["Type alias for `Matrix<2, 4, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x4CP] = $crate::Mat2x4CP<$type>;

            #[doc = @str["Type alias for `Matrix<3, 2, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x2CP] = $crate::Mat3x2CP<$type>;
            #[doc = @str["Type alias for `Matrix<3, 3, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3CP] = $crate::Mat3CP<$type>;
            #[doc = @str["Type alias for `Matrix<3, 4, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x4CP] = $crate::Mat3x4CP<$type>;

            #[doc = @str["Type alias for `Matrix<4, 2, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x2CP] = $crate::Mat4x2CP<$type>;
            #[doc = @str["Type alias for `Matrix<4, 3, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x3CP] = $crate::Mat4x3CP<$type>;
            #[doc = @str["Type alias for `Matrix<4, 4, " ~$type ", VecPacked, ColMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4CP] = $crate::Mat4CP<$type>;

            // Row-Major, Packed

            #[doc = @str["Type alias for `Matrix<2, 2, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2RP] = $crate::Mat2RP<$type>;
            #[doc = @str["Type alias for `Matrix<2, 3, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x3RP] = $crate::Mat2x3RP<$type>;
            #[doc = @str["Type alias for `Matrix<2, 4, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat2x4RP] = $crate::Mat2x4RP<$type>;

            #[doc = @str["Type alias for `Matrix<3, 2, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x2RP] = $crate::Mat3x2RP<$type>;
            #[doc = @str["Type alias for `Matrix<3, 3, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3RP] = $crate::Mat3RP<$type>;
            #[doc = @str["Type alias for `Matrix<3, 4, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat3x4RP] = $crate::Mat3x4RP<$type>;

            #[doc = @str["Type alias for `Matrix<4, 2, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x2RP] = $crate::Mat4x2RP<$type>;
            #[doc = @str["Type alias for `Matrix<4, 3, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4x3RP] = $crate::Mat4x3RP<$type>;
            #[doc = @str["Type alias for `Matrix<4, 4, " ~$type ", VecPacked, RowMajor>`"]]
            $($vis)* type @[~$prefix 'Mat4RP] = $crate::Mat4RP<$type>;
        }
    };
}

/// Expands to a declaration of type specific aabb aliases.
///
/// Syntax:
/// `<vis> <prefix> => <type>`
///
/// Example:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// aabb_aliases!(pub Big => BigInt);
/// ```
///
/// Expands to:
/// ```rust
/// use ggmath::*;
///
/// // Declare a `Scalar` type.
/// type BigInt = i128;
///
/// pub type BigRect = Rect<BigInt>;
/// pub type BigRectP = RectP<BigInt>;
/// pub type BigRectCP = RectCP<BigInt>;
///
/// pub type BigAabb3 = Aabb3<BigInt>;
/// pub type BigAabb3P = Aabb3P<BigInt>;
///
/// // ...
/// ```
#[cfg(feature = "aabb")]
#[macro_export]
macro_rules! aabb_aliases {
    (pub($($vis:tt)*) $prefix:ident => $type:tt) => {
        aabb_aliases! { @(pub($($vis)*)) $prefix => $type }
    };
    (pub $prefix:ident => $type:tt) => {
        aabb_aliases! { @(pub) $prefix => $type }
    };
    ($prefix:ident => $type:tt) => {
        aabb_aliases! { @() $prefix => $type }
    };

    (@($($vis:tt)*) $prefix:ident => $type:tt) => {
        $crate::repetitive! {
            // 2D

            #[doc = @str["Type alias for `Rect<" ~$type ">` / `Aabb<2, " ~$type ", VecAligned, RectCornered>`"]]
            $($vis)* type @[~$prefix 'Rect] = $crate::Rect<$type>;

            #[doc = @str["Type alias for `RectP<" ~$type ">` / `Aabb<2, " ~$type ", VecPacked, RectCornered>`"]]
            $($vis)* type @[~$prefix 'RectP] = $crate::RectP<$type>;

            #[doc = @str["Type alias for `RectC<" ~$type ">` / `Aabb<2, " ~$type ", VecAligned, RectCentered>`"]]
            $($vis)* type @[~$prefix 'RectC] = $crate::RectC<$type>;

            #[doc = @str["Type alias for `RectCP<" ~$type ">` / `Aabb<2, " ~$type ", VecPacked, RectCentered>`"]]
            $($vis)* type @[~$prefix 'RectCP] = $crate::RectCP<$type>;

            #[doc = @str["Type alias for `RectM<" ~$type ">` / `Aabb<2, " ~$type ", VecAligned, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'RectM] = $crate::RectM<$type>;

            #[doc = @str["Type alias for `RectMP<" ~$type ">` / `Aabb<2, " ~$type ", VecPacked, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'RectMP] = $crate::RectMP<$type>;

            // 3D

            #[doc = @str["Type alias for `Aabb3<" ~$type ">` / `Aabb<3, " ~$type ", VecAligned, RectCornered>`"]]
            $($vis)* type @[~$prefix 'Aabb3] = $crate::Aabb3<$type>;

            #[doc = @str["Type alias for `Aabb3P<" ~$type ">` / `Aabb<3, " ~$type ", VecPacked, RectCornered>`"]]
            $($vis)* type @[~$prefix 'Aabb3P] = $crate::Aabb3P<$type>;

            #[doc = @str["Type alias for `Aabb3C<" ~$type ">` / `Aabb<3, " ~$type ", VecAligned, RectCentered>`"]]
            $($vis)* type @[~$prefix 'Aabb3C] = $crate::Aabb3C<$type>;

            #[doc = @str["Type alias for `Aabb3CP<" ~$type ">` / `Aabb<3, " ~$type ", VecPacked, RectCentered>`"]]
            $($vis)* type @[~$prefix 'Aabb3CP] = $crate::Aabb3CP<$type>;

            #[doc = @str["Type alias for `Aabb3M<" ~$type ">` / `Aabb<3, " ~$type ", VecAligned, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'Aabb3M] = $crate::Aabb3M<$type>;

            #[doc = @str["Type alias for `Aabb3MP<" ~$type ">` / `Aabb<3, " ~$type ", VecPacked, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'Aabb3MP] = $crate::Aabb3MP<$type>;

            // 4D

            #[doc = @str["Type alias for `Aabb4<" ~$type ">` / `Aabb<4, " ~$type ", VecAligned, RectCornered>`"]]
            $($vis)* type @[~$prefix 'Aabb4] = $crate::Aabb4<$type>;

            #[doc = @str["Type alias for `Aabb4P<" ~$type ">` / `Aabb<4, " ~$type ", VecPacked, RectCornered>`"]]
            $($vis)* type @[~$prefix 'Aabb4P] = $crate::Aabb4P<$type>;

            #[doc = @str["Type alias for `Aabb4C<" ~$type ">` / `Aabb<4, " ~$type ", VecAligned, RectCentered>`"]]
            $($vis)* type @[~$prefix 'Aabb4C] = $crate::Aabb4C<$type>;

            #[doc = @str["Type alias for `Aabb4CP<" ~$type ">` / `Aabb<4, " ~$type ", VecPacked, RectCentered>`"]]
            $($vis)* type @[~$prefix 'Aabb4CP] = $crate::Aabb4CP<$type>;

            #[doc = @str["Type alias for `Aabb4M<" ~$type ">` / `Aabb<4, " ~$type ", VecAligned, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'Aabb4M] = $crate::Aabb4M<$type>;

            #[doc = @str["Type alias for `Aabb4MP<" ~$type ">` / `Aabb<4, " ~$type ", VecPacked, RectMinMaxed>`"]]
            $($vis)* type @[~$prefix 'Aabb4MP] = $crate::Aabb4MP<$type>;
        }
    };
}
