use core::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::{Scalar, Simdness, Vector};

////////////////////////////////////////////////////////////////////////////////
// Vector2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xy<T: Scalar> {
    /// The 1st component of the vector.
    pub x: T,
    /// The 2nd component of the vector.
    pub y: T,
}

impl<T: Scalar, S: Simdness> Deref for Vector<2, T, S> {
    type Target = Xy<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Vector<2, T, S> is guaranteed to begin with 2 T's, which is Xy<T>
        unsafe { transmute::<&Vector<2, T, S>, &Xy<T>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Vector<2, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Vector<2, T, S> is guaranteed to begin with 2 T's, which is Xy<T>
        unsafe { transmute::<&mut Vector<2, T, S>, &mut Xy<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xyz<T: Scalar> {
    /// The 1st component of the vector.
    pub x: T,
    /// The 2nd component of the vector.
    pub y: T,
    /// The 3rd component of the vector.
    pub z: T,
}

impl<T: Scalar, S: Simdness> Deref for Vector<3, T, S> {
    type Target = Xyz<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Vector<3, T, S> is guaranteed to begin with 3 T's, which is Xyz<T>
        unsafe { transmute::<&Vector<3, T, S>, &Xyz<T>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Vector<3, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Vector<3, T, S> is guaranteed to begin with 3 T's, which is Xyz<T>
        unsafe { transmute::<&mut Vector<3, T, S>, &mut Xyz<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xyzw<T: Scalar> {
    /// The 1st component of the vector.
    pub x: T,
    /// The 2nd component of the vector.
    pub y: T,
    /// The 3rd component of the vector.
    pub z: T,
    /// The 4th component of the vector.
    pub w: T,
}

impl<T: Scalar, S: Simdness> Deref for Vector<4, T, S> {
    type Target = Xyzw<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: Vector<4, T, S> is guaranteed to begin with 4 T's, which is Xyzw<T>
        unsafe { transmute::<&Vector<4, T, S>, &Xyzw<T>>(self) }
    }
}

impl<T: Scalar, S: Simdness> DerefMut for Vector<4, T, S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Vector<4, T, S> is guaranteed to begin with 4 T's, which is Xyzw<T>
        unsafe { transmute::<&mut Vector<4, T, S>, &mut Xyzw<T>>(self) }
    }
}
