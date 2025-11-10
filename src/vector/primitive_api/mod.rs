use crate::Scalar;

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for i128 {}
impl Scalar for isize {}
impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}
impl Scalar for u128 {}
impl Scalar for usize {}
impl Scalar for bool {}

pub(in crate::vector) mod f32_api {
    #[expect(non_camel_case_types)]
    type f = f32;
    include!("float.rs");
}

pub(in crate::vector) mod f64_api {
    #[expect(non_camel_case_types)]
    type f = f64;
    include!("float.rs");
}
