use core::ops::{Deref, DerefMut};

use crate::{
    Alignment, Quaternion, Scalar,
    utils::{transmute_mut, transmute_ref},
};

#[repr(C)]
pub struct Xyzw<T> {
    /// The first component of the quaternion.
    pub x: T,
    /// The second component of the quaternion.
    pub y: T,
    /// The third component of the quaternion.
    pub z: T,
    /// The fourth component of the quaternion.
    pub w: T,
}

impl<T, A: Alignment> Deref for Quaternion<T, A>
where
    T: Scalar,
{
    type Target = Xyzw<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Quaternion<T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_ref::<Quaternion<T, A>, Xyzw<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Quaternion<T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Quaternion<T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_mut::<Quaternion<T, A>, Xyzw<T>>(self) }
    }
}
