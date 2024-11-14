use std::mem::transmute;

use ggmath_proc_macros::swizzles;

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

swizzles!(1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
swizzles!(1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1 1(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

swizzles!(1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
swizzles!(1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1 1(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

swizzles!(1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> T {
                unsafe { self.get_unchecked($($pos), *) }
            }
        )*)*
    }
});
swizzles!(1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<2, T, A> {
                unsafe { self.get_2_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<3, T, A> {
                unsafe { self.get_3_unchecked([$($pos), *]) }
            }
        )*)*
    }
});
swizzles!(1 1 1 1(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self) -> Vector<4, T, A> {
                unsafe { self.get_4_unchecked([$($pos), *]) }
            }
        )*)*
    }
});

// GET MUT

swizzles!(
    sorted unique 1..2(x y: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..2 1..2(x y: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

swizzles!(
    sorted unique 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..3 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..3 1..3 1..3(x y z: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

swizzles!(
    sorted unique 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> &mut vecp!($($len_ident)*) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..4 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);
swizzles!(
    sorted unique 1..4 1..4 1..4 1..4(x y z w: _)_mut => {
        impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
            $($(#[inline(always)]
                pub fn $ident(&mut self) -> ($(&mut vecp!($len_ident)), *) {
                    ($(unsafe { transmute(self.get_mut_unchecked($pos)) }), *)
                }
            )*)*
        }
    }
);

// WITH

swizzles!(unique 1 with_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 with_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

swizzles!(unique 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 with_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

swizzles!(unique 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, value: T) -> Self {
                unsafe { self.with_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<2, T, A>) -> Self {
                unsafe { self.with_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<3, T, A>) -> Self {
                unsafe { self.with_3_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 1 with_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(self, values: Vector<4, T, A>) -> Self {
                unsafe { self.with_4_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

// SET

swizzles!(unique 1 set_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 set_(x y) => {
    impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

swizzles!(unique 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 set_(x y z) => {
    impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});

swizzles!(unique 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, value: T) {
                unsafe { self.set_unchecked($($pos), *, value) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<2, T, A>) {
                unsafe { self.set_2_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<3, T, A>) {
                unsafe { self.set_3_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});
swizzles!(unique 1 1 1 1 set_(x y z w) => {
    impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
        $($(#[inline(always)]
            pub fn $ident(&mut self, values: Vector<4, T, A>) {
                unsafe { self.set_4_unchecked([$($pos), *], values) }
            }
        )*)*
    }
});