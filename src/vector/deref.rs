use core::ops::{Deref, DerefMut};

use crate::{
    transmute_mut, transmute_ref,
    vector::{Alignment, Scalar, Vector},
};

////////////////////////////////////////////////////////////////////////////////
// Vector2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xy<T> {
    /// The 1st element of the vector.
    pub x: T,
    /// The 2nd element of the vector.
    pub y: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<2, T, A> {
    type Target = Xy<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<2, T, A>, Xy<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<2, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<2, T, A>, Xy<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyz<T> {
    /// The 1st element of the vector.
    pub x: T,
    /// The 2nd element of the vector.
    pub y: T,
    /// The 3rd element of the vector.
    pub z: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<3, T, A> {
    type Target = Xyz<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<3, T, A>, Xyz<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<3, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<3, T, A>, Xyz<T>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyzw<T> {
    /// The 1st element of the vector.
    pub x: T,
    /// The 2nd element of the vector.
    pub y: T,
    /// The 3rd element of the vector.
    pub z: T,
    /// The 4th element of the vector.
    pub w: T,
}

impl<T: Scalar, A: Alignment> Deref for Vector<4, T, A> {
    type Target = Xyzw<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref::<Vector<4, T, A>, Xyzw<T>>(self) }
    }
}

impl<T: Scalar, A: Alignment> DerefMut for Vector<4, T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_mut::<Vector<4, T, A>, Xyzw<T>>(self) }
    }
}
