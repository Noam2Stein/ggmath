use crate::vector::{VecAligned, VecPacked, Vector};

pub type Vec2<T> = Vector<2, T, VecAligned>;
pub type Vec3<T> = Vector<3, T, VecAligned>;
pub type Vec4<T> = Vector<4, T, VecAligned>;

pub type Vec2P<T> = Vector<2, T, VecPacked>;
pub type Vec3P<T> = Vector<3, T, VecPacked>;
pub type Vec4P<T> = Vector<4, T, VecPacked>;

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
            $($vis)* type [<$prefix Vec2>] = $crate::aliases::Vec2<$t>;
            $($vis)* type [<$prefix Vec3>] = $crate::aliases::Vec3<$t>;
            $($vis)* type [<$prefix Vec4>] = $crate::aliases::Vec4<$t>;

            $($vis)* type [<$prefix Vec2P>] = $crate::aliases::Vec2P<$t>;
            $($vis)* type [<$prefix Vec3P>] = $crate::aliases::Vec3P<$t>;
            $($vis)* type [<$prefix Vec4P>] = $crate::aliases::Vec4P<$t>;
        }
    };
}
