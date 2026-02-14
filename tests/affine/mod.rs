mod primitive;
mod trait_impl;

macro_rules! test_primitive {
    ($T:ident, $include:literal, $x:expr, $y:expr, $z:expr, $w:expr) => {
        mod $T {
            use ggmath::{Aligned, Unaligned};

            use crate::affine::primitive;

            mod include {
                type T = $T;

                include!($include);
            }

            #[test]
            fn aligned() {
                primitive::test::<$T, Aligned>($x, $y, $z, $w);
                include::test::<Aligned>();
            }

            #[test]
            fn unaligned() {
                primitive::test::<$T, Unaligned>($x, $y, $z, $w);
                include::test::<Unaligned>();
            }
        }
    };
    ($T:ident, $x:expr, $y:expr, $z:expr, $w:expr) => {
        mod $T {
            use ggmath::{Aligned, Unaligned};

            use crate::affine::primitive;

            #[test]
            fn aligned() {
                primitive::test::<$T, Aligned>($x, $y, $z, $w);
            }

            #[test]
            fn unaligned() {
                primitive::test::<$T, Unaligned>($x, $y, $z, $w);
            }
        }
    };
}
test_primitive!(f32, "float.rs", 1.0, 2.0, 3.0, 4.0);
test_primitive!(f64, "float.rs", 1.0, 2.0, 3.0, 4.0);
test_primitive!(i8, 1, 2, 3, 4);
test_primitive!(i16, 1, 2, 3, 4);
test_primitive!(i32, 1, 2, 3, 4);
test_primitive!(i64, 1, 2, 3, 4);
test_primitive!(i128, 1, 2, 3, 4);
test_primitive!(isize, 1, 2, 3, 4);
test_primitive!(u8, 1, 2, 3, 4);
test_primitive!(u16, 1, 2, 3, 4);
test_primitive!(u32, 1, 2, 3, 4);
test_primitive!(u64, 1, 2, 3, 4);
test_primitive!(u128, 1, 2, 3, 4);
test_primitive!(usize, 1, 2, 3, 4);
