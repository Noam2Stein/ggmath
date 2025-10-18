use crate::{Scalar, SimdBehaviour};

mod f32;
mod f64;
mod i32;
mod i64;
mod isize;
mod u32;
mod u64;
mod usize;

////////////////////////////////////////////////////////////////////////////////
// Bool
////////////////////////////////////////////////////////////////////////////////

// All bool vectors are too small for SIMD

impl Scalar for bool {}

impl SimdBehaviour<2> for bool {
    type VectorRepr = [bool; 2];
}

impl SimdBehaviour<3> for bool {
    type VectorRepr = [bool; 3];
}

impl SimdBehaviour<4> for bool {
    type VectorRepr = [bool; 4];
}

////////////////////////////////////////////////////////////////////////////////
// I8
////////////////////////////////////////////////////////////////////////////////

// All i8 vectors are too small for SIMD

impl Scalar for i8 {}

impl SimdBehaviour<2> for i8 {
    type VectorRepr = [i8; 2];
}

impl SimdBehaviour<3> for i8 {
    type VectorRepr = [i8; 3];
}

impl SimdBehaviour<4> for i8 {
    type VectorRepr = [i8; 4];
}

////////////////////////////////////////////////////////////////////////////////
// I16
////////////////////////////////////////////////////////////////////////////////

// All i16 vectors are too small for SIMD

impl Scalar for i16 {}

impl SimdBehaviour<2> for i16 {
    type VectorRepr = [i16; 2];
}

impl SimdBehaviour<3> for i16 {
    type VectorRepr = [i16; 3];
}

impl SimdBehaviour<4> for i16 {
    type VectorRepr = [i16; 4];
}

////////////////////////////////////////////////////////////////////////////////
// I128
////////////////////////////////////////////////////////////////////////////////

// There are currently no SIMD instructions for i128

impl Scalar for i128 {}

impl SimdBehaviour<2> for i128 {
    type VectorRepr = [i128; 2];
}

impl SimdBehaviour<3> for i128 {
    type VectorRepr = [i128; 3];
}

impl SimdBehaviour<4> for i128 {
    type VectorRepr = [i128; 4];
}

////////////////////////////////////////////////////////////////////////////////
// U8
////////////////////////////////////////////////////////////////////////////////

// All u8 vectors are too small for SIMD

impl Scalar for u8 {}

impl SimdBehaviour<2> for u8 {
    type VectorRepr = [u8; 2];
}

impl SimdBehaviour<3> for u8 {
    type VectorRepr = [u8; 3];
}

impl SimdBehaviour<4> for u8 {
    type VectorRepr = [u8; 4];
}

////////////////////////////////////////////////////////////////////////////////
// U16
////////////////////////////////////////////////////////////////////////////////

// All u16 vectors are too small for SIMD

impl Scalar for u16 {}

impl SimdBehaviour<2> for u16 {
    type VectorRepr = [u16; 2];
}

impl SimdBehaviour<3> for u16 {
    type VectorRepr = [u16; 3];
}

impl SimdBehaviour<4> for u16 {
    type VectorRepr = [u16; 4];
}

////////////////////////////////////////////////////////////////////////////////
// U128
////////////////////////////////////////////////////////////////////////////////

// There are currently no SIMD instructions for u128

impl Scalar for u128 {}

impl SimdBehaviour<2> for u128 {
    type VectorRepr = [u128; 2];
}

impl SimdBehaviour<3> for u128 {
    type VectorRepr = [u128; 3];
}

impl SimdBehaviour<4> for u128 {
    type VectorRepr = [u128; 4];
}

////////////////////////////////////////////////////////////////////////////////
// Helper Macros
////////////////////////////////////////////////////////////////////////////////

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! use_core_arch_x86 {
    { $($x:path),* $(,)? } => {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::{$($x),*};
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::{$($x),*};
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use use_core_arch_x86;
