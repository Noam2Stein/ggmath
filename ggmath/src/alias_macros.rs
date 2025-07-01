#[macro_export]
macro_rules! vector_aliases {
    (
        $vis:vis $prefix:ident => $type:ty
    ) => {
        $crate::paste! {
            $vis type [<$prefix Vec2>] = $crate::Vec2<$type>;
            $vis type [<$prefix Vec3>] = $crate::Vec3<$type>;
            $vis type [<$prefix Vec4>] = $crate::Vec4<$type>;

            $vis type [<$prefix Vec2P>] = $crate::Vec2P<$type>;
            $vis type [<$prefix Vec3P>] = $crate::Vec3P<$type>;
            $vis type [<$prefix Vec4P>] = $crate::Vec4P<$type>;
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
        $crate::paste! {
            // Column-Major, Aligned

            $vis type [<$prefix Mat2C>] = $crate::Mat2C<$type>;
            $vis type [<$prefix Mat2x3C>] = $crate::Mat2x3C<$type>;
            $vis type [<$prefix Mat2x4C>] = $crate::Mat2x4C<$type>;

            $vis type [<$prefix Mat3x2C>] = $crate::Mat3x2C<$type>;
            $vis type [<$prefix Mat3C>] = $crate::Mat3C<$type>;
            $vis type [<$prefix Mat3x4C>] = $crate::Mat3x4C<$type>;

            $vis type [<$prefix Mat4x2C>] = $crate::Mat4x2C<$type>;
            $vis type [<$prefix Mat4x3C>] = $crate::Mat4x3C<$type>;
            $vis type [<$prefix Mat4C>] = $crate::Mat4C<$type>;

            // Row-Major, Aligned

            $vis type [<$prefix Mat2R>] = $crate::Mat2R<$type>;
            $vis type [<$prefix Mat2x3R>] = $crate::Mat2x3R<$type>;
            $vis type [<$prefix Mat2x4R>] = $crate::Mat2x4R<$type>;

            $vis type [<$prefix Mat3x2R>] = $crate::Mat3x2R<$type>;
            $vis type [<$prefix Mat3R>] = $crate::Mat3R<$type>;
            $vis type [<$prefix Mat3x4R>] = $crate::Mat3x4R<$type>;

            $vis type [<$prefix Mat4x2R>] = $crate::Mat4x2R<$type>;
            $vis type [<$prefix Mat4x3R>] = $crate::Mat4x3R<$type>;
            $vis type [<$prefix Mat4R>] = $crate::Mat4R<$type>;

            // Column-Major, Packed

            $vis type [<$prefix Mat2CP>] = $crate::Mat2CP<$type>;
            $vis type [<$prefix Mat2x3CP>] = $crate::Mat2x3CP<$type>;
            $vis type [<$prefix Mat2x4CP>] = $crate::Mat2x4CP<$type>;

            $vis type [<$prefix Mat3x2CP>] = $crate::Mat3x2CP<$type>;
            $vis type [<$prefix Mat3CP>] = $crate::Mat3CP<$type>;
            $vis type [<$prefix Mat3x4CP>] = $crate::Mat3x4CP<$type>;

            $vis type [<$prefix Mat4x2CP>] = $crate::Mat4x2CP<$type>;
            $vis type [<$prefix Mat4x3CP>] = $crate::Mat4x3CP<$type>;
            $vis type [<$prefix Mat4CP>] = $crate::Mat4CP<$type>;

            // Row-Major, Packed

            $vis type [<$prefix Mat2RP>] = $crate::Mat2RP<$type>;
            $vis type [<$prefix Mat2x3RP>] = $crate::Mat2x3RP<$type>;
            $vis type [<$prefix Mat2x4RP>] = $crate::Mat2x4RP<$type>;

            $vis type [<$prefix Mat3x2RP>] = $crate::Mat3x2RP<$type>;
            $vis type [<$prefix Mat3RP>] = $crate::Mat3RP<$type>;
            $vis type [<$prefix Mat3x4RP>] = $crate::Mat3x4RP<$type>;

            $vis type [<$prefix Mat4x2RP>] = $crate::Mat4x2RP<$type>;
            $vis type [<$prefix Mat4x3RP>] = $crate::Mat4x3RP<$type>;
            $vis type [<$prefix Mat4RP>] = $crate::Mat4RP<$type>;
        }
    };
}

#[cfg(feature = "rectangle")]
#[macro_export]
macro_rules! rectangle_aliases {
    (
        $(#[$attr:meta])*
        $vis:vis $prefix:ident => $type:ty
    ) => {
        $crate::paste! {
            // Cornered, Aligned

            $vis type [<$prefix Rect2>] = $crate::Rect2<$type>;
            $vis type [<$prefix Rect3>] = $crate::Rect3<$type>;
            $vis type [<$prefix Rect4>] = $crate::Rect4<$type>;

            // Centered, Aligned

            $vis type [<$prefix Rect2C>] = $crate::Rect2C<$type>;
            $vis type [<$prefix Rect3C>] = $crate::Rect3C<$type>;
            $vis type [<$prefix Rect4C>] = $crate::Rect4C<$type>;

            // MinMaxed, Aligned

            $vis type [<$prefix Rect2M>] = $crate::Rect2M<$type>;
            $vis type [<$prefix Rect3M>] = $crate::Rect3M<$type>;
            $vis type [<$prefix Rect4M>] = $crate::Rect4M<$type>;

            // Cornered, Packed

            $vis type [<$prefix Rect2P>] = $crate::Rect2P<$type>;
            $vis type [<$prefix Rect3P>] = $crate::Rect3P<$type>;
            $vis type [<$prefix Rect4P>] = $crate::Rect4P<$type>;

            // Centered, Packed

            $vis type [<$prefix Rect2CP>] = $crate::Rect2CP<$type>;
            $vis type [<$prefix Rect3CP>] = $crate::Rect3CP<$type>;
            $vis type [<$prefix Rect4CP>] = $crate::Rect4CP<$type>;

            // MinMaxed, Packed

            $vis type [<$prefix Rect2MP>] = $crate::Rect2MP<$type>;
            $vis type [<$prefix Rect3MP>] = $crate::Rect3MP<$type>;
            $vis type [<$prefix Rect4MP>] = $crate::Rect4MP<$type>;
        }
    };
}
