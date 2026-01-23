use assert_impl_trait::assert_impl;
use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use ggmath::{Alignment, Length, Scalar, SupportedLength, Vector};

assert_impl!(
    for<const N: usize, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        for<T: Scalar + Pod> {
            Vector<N, T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Vector<N, T, A>: Zeroable,
        }

        Vector<N, f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, bool, A>: Zeroable,
    }
);
