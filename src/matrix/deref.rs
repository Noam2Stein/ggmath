use core::ops::{Deref, DerefMut};

use crate::{
    Alignment, Matrix, Scalar, Vector,
    utils::{transmute_mut, transmute_ref},
};

////////////////////////////////////////////////////////////////////////////////
// Matrix2
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xy<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    pub x_axis: Vector<2, T, A>,
    /// The second column of the matrix.
    pub y_axis: Vector<2, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<2, T, A>
where
    T: Scalar,
{
    type Target = Xy<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_ref::<Matrix<2, T, A>, Xy<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_mut::<Matrix<2, T, A>, Xy<T, A>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix3
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyz<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    pub x_axis: Vector<3, T, A>,
    /// The second column of the matrix.
    pub y_axis: Vector<3, T, A>,
    /// The third column of the matrix.
    pub z_axis: Vector<3, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<3, T, A>
where
    T: Scalar,
{
    type Target = Xyz<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_ref::<Matrix<3, T, A>, Xyz<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_mut::<Matrix<3, T, A>, Xyz<T, A>>(self) }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct Xyzw<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    pub x_axis: Vector<4, T, A>,
    /// The second column of the matrix.
    pub y_axis: Vector<4, T, A>,
    /// The third column of the matrix.
    pub z_axis: Vector<4, T, A>,
    /// The fourth column of the matrix.
    pub w_axis: Vector<4, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<4, T, A>
where
    T: Scalar,
{
    type Target = Xyzw<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_ref::<Matrix<4, T, A>, Xyzw<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_mut::<Matrix<4, T, A>, Xyzw<T, A>>(self) }
    }
}
