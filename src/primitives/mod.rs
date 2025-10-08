use crate::{ElementOfVector, Simd};

mod bool;
mod char;
mod f32;
mod f64;
mod i128;
mod i16;
mod i32;
mod i64;
mod i8;
mod isize;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod usize;

macro_rules! assert_primitive_is_element {
    ($T:ty) => {
        const _: () = {
            const fn helper<
                T: ElementOfVector<0, Simd>
                    + ElementOfVector<1, Simd>
                    + ElementOfVector<2, Simd>
                    + ElementOfVector<3, Simd>
                    + ElementOfVector<4, Simd>
                    + ElementOfVector<5, Simd>
                    + ElementOfVector<6, Simd>
                    + ElementOfVector<7, Simd>
                    + ElementOfVector<8, Simd>,
            >() {
            }

            helper::<$T>();
        };
    };
}
assert_primitive_is_element!(f32);
assert_primitive_is_element!(f64);
assert_primitive_is_element!(i8);
assert_primitive_is_element!(i16);
assert_primitive_is_element!(i32);
assert_primitive_is_element!(i64);
assert_primitive_is_element!(i128);
assert_primitive_is_element!(isize);
assert_primitive_is_element!(u8);
assert_primitive_is_element!(u16);
assert_primitive_is_element!(u32);
assert_primitive_is_element!(u64);
assert_primitive_is_element!(u128);
assert_primitive_is_element!(usize);
assert_primitive_is_element!(bool);
assert_primitive_is_element!(char);
