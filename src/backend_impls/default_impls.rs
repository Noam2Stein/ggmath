use crate::{Alignment, Length, ScalarDefault, SupportedLength};

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

impl<const N: usize, A: Alignment> crate::vector::i8::IntBackend<N, A> for i8 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::i16::IntBackend<N, A> for i16 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::i32::IntBackend<N, A> for i32 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::i64::IntBackend<N, A> for i64 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::i128::IntBackend<N, A> for i128 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::isize::IntBackend<N, A> for isize where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::u8::UintBackend<N, A> for u8 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::u16::UintBackend<N, A> for u16 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::u32::UintBackend<N, A> for u32 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::u64::UintBackend<N, A> for u64 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::u128::UintBackend<N, A> for u128 where
    Length<N>: SupportedLength
{
}

impl<const N: usize, A: Alignment> crate::vector::usize::UintBackend<N, A> for usize where
    Length<N>: SupportedLength
{
}
