use ggmath_proc_macros::swizzles;

use super::*;

swizzles!(1(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($component), *) }
            }
        )*
    }
});
swizzles!(2(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector2<T, A> {
                unsafe { self.get_2_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(3(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector3<T, A> {
                unsafe { self.get_3_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(4(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector4<T, A> {
                unsafe { self.get_4_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(1(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($component), *) }
            }
        )*
    }
});
swizzles!(2(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector2<T, A> {
                unsafe { self.get_2_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(3(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector3<T, A> {
                unsafe { self.get_3_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(4(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector4<T, A> {
                unsafe { self.get_4_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(1(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($component), *) }
            }
        )*
    }
});
swizzles!(2(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector2<T, A> {
                unsafe { self.get_2_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(3(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector3<T, A> {
                unsafe { self.get_3_unchecked([$($component), *]) }
            }
        )*
    }
});
swizzles!(4(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self) -> Vector4<T, A> {
                unsafe { self.get_4_unchecked([$($component), *]) }
            }
        )*
    }
});
