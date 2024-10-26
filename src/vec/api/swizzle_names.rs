use ggmath_proc_macros::{non_repeat_swizzles, swizzles};

use super::*;

// GET

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

// WITH

non_repeat_swizzles!(1 with_(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 with_(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector2<T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($component), *], values) }
            }
        )*
    }
});

non_repeat_swizzles!(1 with_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 with_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector2<T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(3 with_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector3<T, A>) -> Self {
                unsafe { self.with_3_unchecked([$($component), *], values) }
            }
        )*
    }
});

non_repeat_swizzles!(1 with_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 with_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector2<T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(3 with_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector3<T, A>) -> Self {
                unsafe { self.with_3_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(4 with_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(self, values: Vector4<T, A>) -> Self {
                unsafe { self.with_4_unchecked([$($component), *], values) }
            }
        )*
    }
});
