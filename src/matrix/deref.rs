use std::{
    mem::{offset_of, transmute},
    ops::{Deref, DerefMut},
};

use crate::{Matrix, Scalar, Simd, Simdness, Vector};

////////////////////////////////////////////////////////////////////////////////
// Matrix2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xy<T: Scalar, S: Simdness> {
    /// The 1st column of the matrix.
    pub x: Vector<2, T, S>,
    /// The 2nd column of the matrix.
    pub y: Vector<2, T, S>,
}

impl<T: Scalar, S: Simdness> Deref for Matrix<2, T, S> {
    type Target = Xy<T, S>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Matrix<2, T, S> is guaranteed to begin with 2 vec2's.
        // The offset of the y vector is guaranteed to be `size_of::<T>() * 2`,
        // which correctly maps to the `zw` of the internal mat2 as vec4 repr.
        unsafe { transmute::<&Matrix<2, T, S>, &Xy<T, S>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Matrix<2, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Matrix<2, T, S> is guaranteed to begin with 2 vec2's.
        // The offset of the y vector is guaranteed to be `size_of::<T>() * 2`,
        // which correctly maps to the `zw` of the internal mat2 as vec4 repr.
        unsafe { transmute::<&mut Matrix<2, T, S>, &mut Xy<T, S>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Matrix3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyz<T: Scalar, S: Simdness> {
    /// The 1st column of the matrix.
    pub x: Vector<3, T, S>,
    /// The 2nd column of the matrix.
    pub y: Vector<3, T, S>,
    /// The 3rd column of the matrix.
    pub z: Vector<3, T, S>,
}

impl<T: Scalar, S: Simdness> Deref for Matrix<3, T, S> {
    type Target = Xyz<T, S>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Matrix<3, T, S> is guaranteed to begin with 3 vec3's.
        unsafe { transmute::<&Matrix<3, T, S>, &Xyz<T, S>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Matrix<3, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Matrix<3, T, S> is guaranteed to begin with 3 vec3's.
        unsafe { transmute::<&mut Matrix<3, T, S>, &mut Xyz<T, S>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Matrix4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyzw<T: Scalar, S: Simdness> {
    /// The 1st column of the matrix.
    pub x: Vector<4, T, S>,
    /// The 2nd column of the matrix.
    pub y: Vector<4, T, S>,
    /// The 3rd column of the matrix.
    pub z: Vector<4, T, S>,
    /// The 4th column of the matrix.
    pub w: Vector<4, T, S>,
}

impl<T: Scalar, S: Simdness> Deref for Matrix<4, T, S> {
    type Target = Xyzw<T, S>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Matrix<4, T, S> is guaranteed to begin with 4 vec4's.
        unsafe { transmute::<&Matrix<4, T, S>, &Xyzw<T, S>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Ensure Safety
////////////////////////////////////////////////////////////////////////////////

const _: () = assert!(offset_of!(Xy<f32, Simd>, y) == size_of::<f32>() * 2);
const _: () = assert!(offset_of!(Xy<f64, Simd>, y) == size_of::<f64>() * 2);
const _: () = assert!(offset_of!(Xy<i8, Simd>, y) == size_of::<i8>() * 2);
const _: () = assert!(offset_of!(Xy<i16, Simd>, y) == size_of::<i16>() * 2);
const _: () = assert!(offset_of!(Xy<i32, Simd>, y) == size_of::<i32>() * 2);
const _: () = assert!(offset_of!(Xy<i64, Simd>, y) == size_of::<i64>() * 2);
const _: () = assert!(offset_of!(Xy<isize, Simd>, y) == size_of::<isize>() * 2);
const _: () = assert!(offset_of!(Xy<u8, Simd>, y) == size_of::<u8>() * 2);
const _: () = assert!(offset_of!(Xy<u16, Simd>, y) == size_of::<u16>() * 2);
const _: () = assert!(offset_of!(Xy<u32, Simd>, y) == size_of::<u32>() * 2);
const _: () = assert!(offset_of!(Xy<u64, Simd>, y) == size_of::<u64>() * 2);
const _: () = assert!(offset_of!(Xy<usize, Simd>, y) == size_of::<usize>() * 2);
const _: () = assert!(offset_of!(Xy<bool, Simd>, y) == size_of::<bool>() * 2);
