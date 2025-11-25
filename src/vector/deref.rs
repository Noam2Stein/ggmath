use core::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::{Alignment, Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
// Vector2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xy<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<2, T, A> {
    type Target = Xy<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: The layout of `Xy<T>` is the same as `[T; 2]`.
        unsafe { transmute::<&[T; 2], &Xy<T>>(self.as_array_ref()) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<2, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The layout of `Xy<T>` is the same as `[T; 2]`.
        unsafe { transmute::<&mut [T; 2], &mut Xy<T>>(self.as_array_mut()) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyz<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<3, T, A> {
    type Target = Xyz<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: The layout of `Xyz<T>` is the same as `[T; 3]`.
        unsafe { transmute::<&[T; 3], &Xyz<T>>(self.as_array_ref()) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<3, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The layout of `Xyz<T>` is the same as `[T; 3]`.
        unsafe { transmute::<&mut [T; 3], &mut Xyz<T>>(self.as_array_mut()) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyzw<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
    /// The fourth element of the vector.
    pub w: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<4, T, A> {
    type Target = Xyzw<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: The layout of `Xyzw<T>` is the same as `[T; 4]`.
        unsafe { transmute::<&[T; 4], &Xyzw<T>>(self.as_array_ref()) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<4, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The layout of `Xyzw<T>` is the same as `[T; 4]`.
        unsafe { transmute::<&mut [T; 4], &mut Xyzw<T>>(self.as_array_mut()) }
    }
}
