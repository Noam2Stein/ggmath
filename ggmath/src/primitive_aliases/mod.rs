//! Type aliases for primitives.
//! For example for (```FVec3```, ```IVec2```, ```BVecN<N>```

use super::*;

use ggmath_proc_macros::scalar_aliases;

scalar_aliases!(pub mod f32 for f32(F));
scalar_aliases!(pub mod f64 for f64(D));

scalar_aliases!(pub mod u8 for u8(U8));
scalar_aliases!(pub mod u16 for u16(U16));
scalar_aliases!(pub mod u32 for u32(U));
scalar_aliases!(pub mod u64 for u64(U64));
scalar_aliases!(pub mod u128 for u128(U128));
scalar_aliases!(pub mod usize for usize(USize));

scalar_aliases!(pub mod i8 for i8(I8));
scalar_aliases!(pub mod i16 for i16(I16));
scalar_aliases!(pub mod i32 for i32(I));
scalar_aliases!(pub mod i64 for i64(I64));
scalar_aliases!(pub mod i128 for i128(I128));
scalar_aliases!(pub mod isize for isize(ISize));

scalar_aliases!(pub mod bool for bool(B));
