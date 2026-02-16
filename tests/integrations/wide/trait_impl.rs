use assert_impl_trait::assert_impl;
use ggmath::{
    Alignment, Length, Scalar, SupportedLength,
    constants::{Infinity, Max, Min, Nan, NegInfinity, NegOne, One, Zero},
};
use wide::{
    f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8,
    i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2,
    u64x4, u64x8,
};

assert_impl!(
    for<const N: usize, Frac, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        f32x4: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f32x8: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f32x16: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x2: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x4: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x8: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        i8x16: Scalar + Zero + One + NegOne + Min + Max,
        i8x32: Scalar + Zero + One + NegOne + Min + Max,
        i16x8: Scalar + Zero + One + NegOne + Min + Max,
        i16x16: Scalar + Zero + One + NegOne + Min + Max,
        i16x32: Scalar + Zero + One + NegOne + Min + Max,
        i32x4: Scalar + Zero + One + NegOne + Min + Max,
        i32x8: Scalar + Zero + One + NegOne + Min + Max,
        i32x16: Scalar + Zero + One + NegOne + Min + Max,
        i64x2: Scalar + Zero + One + NegOne + Min + Max,
        i64x4: Scalar + Zero + One + NegOne + Min + Max,
        i64x8: Scalar + Zero + One + NegOne + Min + Max,
        u8x16: Scalar + Zero + One + Min + Max,
        u8x32: Scalar + Zero + One + Min + Max,
        u16x8: Scalar + Zero + One + Min + Max,
        u16x16: Scalar + Zero + One + Min + Max,
        u16x32: Scalar + Zero + One + Min + Max,
        u32x4: Scalar + Zero + One + Min + Max,
        u32x8: Scalar + Zero + One + Min + Max,
        u32x16: Scalar + Zero + One + Min + Max,
        u64x2: Scalar + Zero + One + Min + Max,
        u64x4: Scalar + Zero + One + Min + Max,
        u64x8: Scalar + Zero + One + Min + Max,
    }
);
