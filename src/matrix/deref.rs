use std::ops::{Deref, DerefMut};

use crate::{Matrix, Scalar, Simdness, Vector};

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
        // SAFETY: Xy is 2 vector2s
        unsafe { core::mem::transmute::<&[Vector<2, T, S>; 2], &Xy<T, S>>(self.as_vectors_ref()) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Matrix<2, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Xy is 2 vector2s
        unsafe {
            core::mem::transmute::<&mut [Vector<2, T, S>; 2], &mut Xy<T, S>>(self.as_vectors_mut())
        }
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
        // SAFETY: Xyz is 3 vector3s
        unsafe { core::mem::transmute::<&Matrix<3, T, S>, &Xyz<T, S>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Matrix<3, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Xyz is 3 vector3s
        unsafe { core::mem::transmute::<&mut Matrix<3, T, S>, &mut Xyz<T, S>>(self) }
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
        // SAFETY: Xyzw is 4 vector4s
        unsafe { core::mem::transmute::<&Matrix<4, T, S>, &Xyzw<T, S>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Matrix<4, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Xyzw is 4 vector4s
        unsafe { core::mem::transmute::<&mut Matrix<4, T, S>, &mut Xyzw<T, S>>(self) }
    }
}
