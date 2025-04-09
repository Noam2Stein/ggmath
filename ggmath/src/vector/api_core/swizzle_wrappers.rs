use std::mem::transmute;

use ggmath_proc_macros::for_swizzles;

use super::*;

macro_rules! vecp {
    (_1) => {
        T
    };
    (_2) => {
        Vec2P<T>
    };
    (_3) => {
        Vec3P<T>
    };
    (_4) => {
        Vec4P<T>
    };
}

// GET

for_swizzles!(1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
for_swizzles!(1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_1_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

for_swizzles!(1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
for_swizzles!(1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_1_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

for_swizzles!(1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
for_swizzles!(1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
for_swizzles!(1 1 1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_1_1_1_1_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

// GET MUT

for_swizzles!(
    sorted unique 1..2(x y: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..2 1..2(x y: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

for_swizzles!(
    sorted unique 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..3 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..3 1..3 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

for_swizzles!(
    sorted unique 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..4 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
for_swizzles!(
    sorted unique 1..4 1..4 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub const fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

// WITH

for_swizzles!(unique 1 with_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 with_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

for_swizzles!(unique 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

for_swizzles!(unique 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_1_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

// SET

for_swizzles!(unique 1 set_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 set_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

for_swizzles!(unique 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

for_swizzles!(unique 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
for_swizzles!(unique 1 1 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_1_1_1_1_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
