mod primitive;
mod trait_impl;

macro_rules! test_primitive {
    ($T:ident, $include:literal, $x:expr, $y:expr, $z:expr, $w:expr) => {
        mod $T {
            use ggmath::{Aligned, Unaligned};

            use crate::vector::primitive;

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
}
test_primitive!(f32, "float.rs", 1.0, 2.0, 3.0, 4.0);
test_primitive!(f64, "float.rs", 1.0, 2.0, 3.0, 4.0);
test_primitive!(i8, "int.rs", 1, 2, 3, 4);
test_primitive!(i16, "int.rs", 1, 2, 3, 4);
test_primitive!(i32, "int.rs", 1, 2, 3, 4);
test_primitive!(i64, "int.rs", 1, 2, 3, 4);
test_primitive!(i128, "int.rs", 1, 2, 3, 4);
test_primitive!(isize, "int.rs", 1, 2, 3, 4);
test_primitive!(u8, "uint.rs", 1, 2, 3, 4);
test_primitive!(u16, "uint.rs", 1, 2, 3, 4);
test_primitive!(u32, "uint.rs", 1, 2, 3, 4);
test_primitive!(u64, "uint.rs", 1, 2, 3, 4);
test_primitive!(u128, "uint.rs", 1, 2, 3, 4);
test_primitive!(usize, "uint.rs", 1, 2, 3, 4);
