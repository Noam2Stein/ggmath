#[macro_export]
macro_rules! vector_aliases {
    (
        $vis:vis $prefix:ident => $type:ty
    ) => {
        $crate::macro_loop! {
            #[doc = concat!("Type alias for `Vector<2, ", stringify!($type), ", VecAligned>`")]
            $vis type @[$prefix Vec2] = $crate::Vec2<$type>;

            #[doc = concat!("Type alias for `Vector<3, ", stringify!($type), ", VecAligned>`")]
            $vis type @[$prefix Vec3] = $crate::Vec3<$type>;

            #[doc = concat!("Type alias for `Vector<4, ", stringify!($type), ", VecAligned>`")]
            $vis type @[$prefix Vec4] = $crate::Vec4<$type>;

            #[doc = concat!("Type alias for `Vector<2, ", stringify!($type), ", VecPacked>`")]
            $vis type @[$prefix Vec2P] = $crate::Vec2P<$type>;

            #[doc = concat!("Type alias for `Vector<3, ", stringify!($type), ", VecPacked>`")]
            $vis type @[$prefix Vec3P] = $crate::Vec3P<$type>;

            #[doc = concat!("Type alias for `Vector<4, ", stringify!($type), ", VecPacked>`")]
            $vis type @[$prefix Vec4P] = $crate::Vec4P<$type>;
        }
    };
}

#[cfg(feature = "matrix")]
#[macro_export]
macro_rules! matrix_aliases {
    (
        $(#[$attr:meta])*
        $vis:vis $prefix:ident => $type:ty
    ) => {
        $crate::macro_loop! {
            // Column-Major, Aligned

            #[doc = concat!("Type alias for `Matrix<2, 2, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat2C] = $crate::Mat2C<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 3, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat2x3C] = $crate::Mat2x3C<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 4, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat2x4C] = $crate::Mat2x4C<$type>;

            #[doc = concat!("Type alias for `Matrix<3, 2, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat3x2C] = $crate::Mat3x2C<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 3, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat3C] = $crate::Mat3C<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 4, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat3x4C] = $crate::Mat3x4C<$type>;

            #[doc = concat!("Type alias for `Matrix<4, 2, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat4x2C] = $crate::Mat4x2C<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 3, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat4x3C] = $crate::Mat4x3C<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 4, ", stringify!($type), ", VecAligned, ColMajor>`")]
            $vis type @[$prefix Mat4C] = $crate::Mat4C<$type>;

            // Row-Major, Aligned

            #[doc = concat!("Type alias for `Matrix<2, 2, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat2R] = $crate::Mat2R<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 3, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat2x3R] = $crate::Mat2x3R<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 4, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat2x4R] = $crate::Mat2x4R<$type>;

            #[doc = concat!("Type alias for `Matrix<3, 2, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat3x2R] = $crate::Mat3x2R<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 3, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat3R] = $crate::Mat3R<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 4, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat3x4R] = $crate::Mat3x4R<$type>;

            #[doc = concat!("Type alias for `Matrix<4, 2, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat4x2R] = $crate::Mat4x2R<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 3, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat4x3R] = $crate::Mat4x3R<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 4, ", stringify!($type), ", VecAligned, RowMajor>`")]
            $vis type @[$prefix Mat4R] = $crate::Mat4R<$type>;

            // Column-Major, Packed

            #[doc = concat!("Type alias for `Matrix<2, 2, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat2CP] = $crate::Mat2CP<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 3, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat2x3CP] = $crate::Mat2x3CP<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 4, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat2x4CP] = $crate::Mat2x4CP<$type>;

            #[doc = concat!("Type alias for `Matrix<3, 2, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat3x2CP] = $crate::Mat3x2CP<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 3, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat3CP] = $crate::Mat3CP<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 4, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat3x4CP] = $crate::Mat3x4CP<$type>;

            #[doc = concat!("Type alias for `Matrix<4, 2, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat4x2CP] = $crate::Mat4x2CP<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 3, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat4x3CP] = $crate::Mat4x3CP<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 4, ", stringify!($type), ", VecPacked, ColMajor>`")]
            $vis type @[$prefix Mat4CP] = $crate::Mat4CP<$type>;

            // Row-Major, Packed

            #[doc = concat!("Type alias for `Matrix<2, 2, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat2RP] = $crate::Mat2RP<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 3, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat2x3RP] = $crate::Mat2x3RP<$type>;
            #[doc = concat!("Type alias for `Matrix<2, 4, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat2x4RP] = $crate::Mat2x4RP<$type>;

            #[doc = concat!("Type alias for `Matrix<3, 2, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat3x2RP] = $crate::Mat3x2RP<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 3, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat3RP] = $crate::Mat3RP<$type>;
            #[doc = concat!("Type alias for `Matrix<3, 4, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat3x4RP] = $crate::Mat3x4RP<$type>;

            #[doc = concat!("Type alias for `Matrix<4, 2, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat4x2RP] = $crate::Mat4x2RP<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 3, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat4x3RP] = $crate::Mat4x3RP<$type>;
            #[doc = concat!("Type alias for `Matrix<4, 4, ", stringify!($type), ", VecPacked, RowMajor>`")]
            $vis type @[$prefix Mat4RP] = $crate::Mat4RP<$type>;
        }
    };
}

#[cfg(feature = "aabb")]
#[macro_export]
macro_rules! rectangle_aliases {
    (
        $(#[$attr:meta])*
        $vis:vis $prefix:ident => $type:ty
    ) => {
        $crate::macro_loop! {
            // 2D

            #[doc = concat!("Type alias for `Rect<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecAligned, RectCornered>`")]
            $vis type @[$prefix Rect] = $crate::Rect<$type>;

            #[doc = concat!("Type alias for `RectP<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecPacked, RectCornered>`")]
            $vis type @[$prefix RectP] = $crate::RectP<$type>;

            #[doc = concat!("Type alias for `RectC<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecAligned, RectCentered>`")]
            $vis type @[$prefix RectC] = $crate::RectC<$type>;

            #[doc = concat!("Type alias for `RectCP<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecPacked, RectCentered>`")]
            $vis type @[$prefix RectCP] = $crate::RectCP<$type>;

            #[doc = concat!("Type alias for `RectM<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecAligned, RectMinMaxed>`")]
            $vis type @[$prefix RectM] = $crate::RectM<$type>;

            #[doc = concat!("Type alias for `RectMP<", stringify!($type), ">` / `Aabb<2, ", stringify!($type), ", VecPacked, RectMinMaxed>`")]
            $vis type @[$prefix RectMP] = $crate::RectMP<$type>;

            // 3D

            #[doc = concat!("Type alias for `Box<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecAligned, RectCornered>`")]
            $vis type @[$prefix Box] = $crate::Box<$type>;

            #[doc = concat!("Type alias for `BoxP<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecPacked, RectCornered>`")]
            $vis type @[$prefix BoxP] = $crate::BoxP<$type>;

            #[doc = concat!("Type alias for `BoxC<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecAligned, RectCentered>`")]
            $vis type @[$prefix BoxC] = $crate::BoxC<$type>;

            #[doc = concat!("Type alias for `BoxCP<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecPacked, RectCentered>`")]
            $vis type @[$prefix BoxCP] = $crate::BoxCP<$type>;

            #[doc = concat!("Type alias for `BoxM<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecAligned, RectMinMaxed>`")]
            $vis type @[$prefix BoxM] = $crate::BoxM<$type>;

            #[doc = concat!("Type alias for `BoxMP<", stringify!($type), ">` / `Aabb<3, ", stringify!($type), ", VecPacked, RectMinMaxed>`")]
            $vis type @[$prefix BoxMP] = $crate::BoxMP<$type>;

            // 4D

            #[doc = concat!("Type alias for `Aabb4<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecAligned, RectCornered>`")]
            $vis type @[$prefix Aabb4] = $crate::Aabb4<$type>;

            #[doc = concat!("Type alias for `Aabb4P<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecPacked, RectCornered>`")]
            $vis type @[$prefix Aabb4P] = $crate::Aabb4P<$type>;

            #[doc = concat!("Type alias for `Aabb4C<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecAligned, RectCentered>`")]
            $vis type @[$prefix Aabb4C] = $crate::Aabb4C<$type>;

            #[doc = concat!("Type alias for `Aabb4CP<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecPacked, RectCentered>`")]
            $vis type @[$prefix Aabb4CP] = $crate::Aabb4CP<$type>;

            #[doc = concat!("Type alias for `Aabb4M<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecAligned, RectMinMaxed>`")]
            $vis type @[$prefix Aabb4M] = $crate::Aabb4M<$type>;

            #[doc = concat!("Type alias for `Aabb4MP<", stringify!($type), ">` / `Aabb<4, ", stringify!($type), ", VecPacked, RectMinMaxed>`")]
            $vis type @[$prefix Aabb4MP] = $crate::Aabb4MP<$type>;
        }
    };
}
