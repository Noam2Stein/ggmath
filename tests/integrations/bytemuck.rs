use assert_impl_trait::assert_impl;
use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use ggmath::{Alignment, Length, Mask, Quaternion, Scalar, SupportedLength, Vector};

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

        for<T: Scalar + Pod> {
            Quaternion<T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Quaternion<T, A>: Zeroable,
        }

        Quaternion<f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<bool, A>: Zeroable,

        for<T: Scalar + 'static> {
            Mask<N, T, A>: NoUninit,
        }
        for<T: Scalar> {
            Mask<N, T, A>: Zeroable,
        }
    }
);
