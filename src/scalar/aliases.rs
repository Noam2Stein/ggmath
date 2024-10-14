use crate as gomath;

use gomath_proc_macros::scalar_aliases;

scalar_aliases!(pub mod f32: F);
scalar_aliases!(pub mod f64: D);

scalar_aliases!(pub mod u8: U8);
scalar_aliases!(pub mod u16: U16);
scalar_aliases!(pub mod u32: U);
scalar_aliases!(pub mod u64: U64);
scalar_aliases!(pub mod u128: U128);
scalar_aliases!(pub mod usize: USize);

scalar_aliases!(pub mod i8: I8);
scalar_aliases!(pub mod i16: I16);
scalar_aliases!(pub mod i32: I);
scalar_aliases!(pub mod i64: I64);
scalar_aliases!(pub mod i128: I128);
scalar_aliases!(pub mod isize: ISize);

scalar_aliases!(pub mod bool: B);
