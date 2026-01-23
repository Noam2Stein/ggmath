use core::ops::{Deref, DerefMut};

use crate::{
    Alignment, Scalar, Vector,
    utils::{transmute_mut, transmute_ref},
};

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

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<2, T, A>, Xy<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<2, T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<2, T, A>, Xy<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyz<T> {
    /// The second element of the vector.
    pub x: T,
    /// The third element of the vector.
    pub y: T,
    /// The fourth element of the vector.
    pub z: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<3, T, A> {
    type Target = Xyz<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<3, T, A>, Xyz<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<3, T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<3, T, A>, Xyz<T>>(self) }
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<4, T, A>, Xyzw<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<4, T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<4, T, A>, Xyzw<T>>(self) }
    }
}
