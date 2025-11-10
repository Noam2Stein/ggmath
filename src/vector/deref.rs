use core::{
    mem::transmute,
    ops::{Deref, DerefMut},
};

use crate::{Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
// Vector2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xy<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T: Scalar> Deref for Vector<2, T> {
    type Target = Xy<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe because `Vector<2, T>` is guaranteed to begin
        // with `[T; 2]`, which is identical to `Xy<T>`.
        unsafe { transmute::<&Vector<2, T>, &Xy<T>>(self) }
    }
}

impl<T: Scalar> DerefMut for Vector<2, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe because `Vector<2, T>` is guaranteed to begin
        // with `[T; 2]`, which is identical to `Xy<T>`.
        unsafe { transmute::<&mut Vector<2, T>, &mut Xy<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xyz<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Deref for Vector<3, T> {
    type Target = Xyz<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe because `Vector<3, T>` is guaranteed to begin
        // with `[T; 3]`, which is identical to `Xyz<T>`.
        unsafe { transmute::<&Vector<3, T>, &Xyz<T>>(self) }
    }
}

impl<T: Scalar> DerefMut for Vector<3, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe because `Vector<3, T>` is guaranteed to begin
        // with `[T; 3]`, which is identical to `Xyz<T>`.
        unsafe { transmute::<&mut Vector<3, T>, &mut Xyz<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xyzw<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Scalar> Deref for Vector<4, T> {
    type Target = Xyzw<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe because `Vector<4, T>` is guaranteed to begin
        // with `[T; 4]`, which is identical to `Xyzw<T>`.
        unsafe { transmute::<&Vector<4, T>, &Xyzw<T>>(self) }
    }
}

impl<T: Scalar> DerefMut for Vector<4, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: This is safe because `Vector<4, T>` is guaranteed to begin
        // with `[T; 4]`, which is identical to `Xyzw<T>`.
        unsafe { transmute::<&mut Vector<4, T>, &mut Xyzw<T>>(self) }
    }
}
