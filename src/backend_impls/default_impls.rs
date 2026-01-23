use crate::{Alignment, ScalarDefault, Length, SupportedLength};

impl ScalarDefault for f64 {}
impl ScalarDefault for i8 {}
impl ScalarDefault for i16 {}
impl ScalarDefault for i32 {}
impl ScalarDefault for i64 {}
impl ScalarDefault for i128 {}
impl ScalarDefault for isize {}
impl ScalarDefault for u8 {}
impl ScalarDefault for u16 {}
impl ScalarDefault for u32 {}
impl ScalarDefault for u64 {}
impl ScalarDefault for u128 {}
impl ScalarDefault for usize {}
impl ScalarDefault for bool {}

impl<const N: usize, A: Alignment> crate::vector::f64::FloatBackend<N, A> for f64 where
    Length<N>: SupportedLength
{
}
