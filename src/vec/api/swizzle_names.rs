use std::mem::transmute;

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

// GET MUT

impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
    #[inline(always)]
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(0) }
    }
    #[inline(always)]
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(1) }
    }

    #[inline(always)]
    pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.y_mut())) }
    }

    #[inline(always)]
    pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.x_mut()) }
    }
}
impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
    #[inline(always)]
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(0) }
    }
    #[inline(always)]
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(1) }
    }
    #[inline(always)]
    pub fn z_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(2) }
    }

    #[inline(always)]
    pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.y_mut())) }
    }
    #[inline(always)]
    pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.z_mut())) }
    }
    #[inline(always)]
    pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.y_mut()), transmute(self.z_mut())) }
    }

    #[inline(always)]
    pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.x_mut()),
                transmute(self.y_mut()),
                transmute(self.z_mut()),
            )
        }
    }

    #[inline(always)]
    pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
        unsafe { transmute(self.x_y_mut()) }
    }

    #[inline(always)]
    pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.x_mut()) }
    }
    #[inline(always)]
    pub fn yz_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.y_mut()) }
    }

    #[inline(always)]
    pub fn xy_z_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
        unsafe { transmute(self.x_y_mut()) }
    }
}
impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
    #[inline(always)]
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(0) }
    }
    #[inline(always)]
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(1) }
    }
    #[inline(always)]
    pub fn z_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(2) }
    }
    #[inline(always)]
    pub fn w_mut(&mut self) -> &mut T {
        unsafe { self.get_mut_unchecked(3) }
    }

    #[inline(always)]
    pub fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.y_mut())) }
    }
    #[inline(always)]
    pub fn x_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.z_mut())) }
    }
    #[inline(always)]
    pub fn x_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.x_mut()), transmute(self.w_mut())) }
    }
    #[inline(always)]
    pub fn y_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.y_mut()), transmute(self.z_mut())) }
    }
    #[inline(always)]
    pub fn y_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.y_mut()), transmute(self.w_mut())) }
    }
    #[inline(always)]
    pub fn z_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe { (transmute(self.z_mut()), transmute(self.w_mut())) }
    }

    #[inline(always)]
    pub fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.x_mut()),
                transmute(self.y_mut()),
                transmute(self.z_mut()),
            )
        }
    }
    #[inline(always)]
    pub fn x_y_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.x_mut()),
                transmute(self.y_mut()),
                transmute(self.w_mut()),
            )
        }
    }
    #[inline(always)]
    pub fn x_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.x_mut()),
                transmute(self.z_mut()),
                transmute(self.w_mut()),
            )
        }
    }
    #[inline(always)]
    pub fn y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.y_mut()),
                transmute(self.z_mut()),
                transmute(self.w_mut()),
            )
        }
    }

    #[inline(always)]
    pub fn x_y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        unsafe {
            (
                transmute(self.x_mut()),
                transmute(self.y_mut()),
                transmute(self.z_mut()),
                transmute(self.w_mut()),
            )
        }
    }

    #[inline(always)]
    pub fn xy_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.x_mut()) }
    }
    #[inline(always)]
    pub fn yz_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.y_mut()) }
    }
    #[inline(always)]
    pub fn zw_mut(&mut self) -> &mut Vec2P<T> {
        unsafe { transmute(self.z_mut()) }
    }

    #[inline(always)]
    pub fn xy_z_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
        unsafe { transmute(self.x_z_mut()) }
    }
    #[inline(always)]
    pub fn xy_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
        unsafe { transmute(self.x_w_mut()) }
    }
    #[inline(always)]
    pub fn yz_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T) {
        unsafe { transmute(self.y_w_mut()) }
    }

    #[inline(always)]
    pub fn xy_z_w_mut(&mut self) -> (&mut Vec2P<T>, &mut T, &mut T) {
        unsafe { transmute(self.x_z_w_mut()) }
    }

    #[inline(always)]
    pub fn x_yz_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
        unsafe { transmute(self.x_y_mut()) }
    }
    #[inline(always)]
    pub fn x_zw_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
        unsafe { transmute(self.x_z_mut()) }
    }
    #[inline(always)]
    pub fn y_zw_mut(&mut self) -> (&mut T, &mut Vec2P<T>) {
        unsafe { transmute(self.y_z_mut()) }
    }

    #[inline(always)]
    pub fn x_yz_w_mut(&mut self) -> (&mut T, &mut Vec2P<T>, &mut T) {
        unsafe { transmute(self.x_y_w_mut()) }
    }

    #[inline(always)]
    pub fn x_y_zw_mut(&mut self) -> (&mut T, &mut T, &mut Vec2P<T>) {
        unsafe { transmute(self.x_y_z_mut()) }
    }

    #[inline(always)]
    pub fn xyz_mut(&mut self) -> &mut Vec3P<T> {
        unsafe { transmute(self.x_mut()) }
    }
    #[inline(always)]
    pub fn yzw_mut(&mut self) -> &mut Vec3P<T> {
        unsafe { transmute(self.y_mut()) }
    }

    #[inline(always)]
    pub fn xyz_w_mut(&mut self) -> (&mut Vec3P<T>, &mut T) {
        unsafe { transmute(self.x_w_mut()) }
    }

    #[inline(always)]
    pub fn x_yzw_mut(&mut self) -> (&mut T, &mut Vec3P<T>) {
        unsafe { transmute(self.x_y_mut()) }
    }
}

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

// SET

non_repeat_swizzles!(1 set_(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 set_(x, y) => {
    impl<T: Scalar, A: VecAlignment> Vector2<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector2<T, A>) {
                unsafe { self.set_2_unchecked([$($component), *], values) }
            }
        )*
    }
});

non_repeat_swizzles!(1 set_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 set_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector2<T, A>) {
                unsafe { self.set_2_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(3 set_(x, y, z) => {
    impl<T: Scalar, A: VecAlignment> Vector3<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector3<T, A>) {
                unsafe { self.set_3_unchecked([$($component), *], values) }
            }
        )*
    }
});

non_repeat_swizzles!(1 set_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($component), *, value) }
            }
        )*
    }
});
non_repeat_swizzles!(2 set_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector2<T, A>) {
                unsafe { self.set_2_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(3 set_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector3<T, A>) {
                unsafe { self.set_3_unchecked([$($component), *], values) }
            }
        )*
    }
});
non_repeat_swizzles!(4 set_(x, y, z, w) => {
    impl<T: Scalar, A: VecAlignment> Vector4<T, A> {
        $(
            #[inline(always)]
            pub fn $ident(&mut self, values: Vector4<T, A>) {
                unsafe { self.set_4_unchecked([$($component), *], values) }
            }
        )*
    }
});
