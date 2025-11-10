use crate::{Scalar, SupportedVecLen, VecLen, Vector};

#[macro_export]
macro_rules! vec2 {
    ($($arg:expr),* $(,)?) => {
        $crate::Vector::<2, _>::from(($($arg,)*))
    };
}

pub use vec2;

#[macro_export]
macro_rules! vec3 {
    ($($arg:expr),* $(,)?) => {
        $crate::Vector::<3, _>::from(($($arg,)*))
    };
}

pub use vec3;

#[macro_export]
macro_rules! vec4 {
    ($($arg:expr),* $(,)?) => {
        $crate::Vector::<4, _>::from(($($arg,)*))
    };
}

pub use vec4;

////////////////////////////////////////////////////////////////////////////////
// Constructor Impls
////////////////////////////////////////////////////////////////////////////////

impl<const N: usize, T: Scalar> From<(T,)> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Vector::splat(value.0)
    }
}

impl<const N: usize, T: Scalar> From<([T; N],)> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(value: ([T; N],)) -> Self {
        Vector::from_array(value.0)
    }
}

impl<const N: usize, T: Scalar> From<(Vector<N, T>,)> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(value: (Vector<N, T>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vec2 Constructor Impls
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar> From<(T, T)> for Vector<2, T> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Vector::from_array([value.0, value.1])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vec3 Constructor Impls
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar> From<(T, T, T)> for Vector<3, T> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Vector::from_array([value.0, value.1, value.2])
    }
}

impl<T: Scalar> From<(T, Vector<2, T>)> for Vector<3, T> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T>)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1]])
    }
}

impl<T: Scalar> From<(Vector<2, T>, T)> for Vector<3, T> {
    #[inline(always)]
    fn from(value: (Vector<2, T>, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vec4 Constructor Impls
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar> From<(T, T, T, T)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Vector::from_array([value.0, value.1, value.2, value.3])
    }
}

impl<T: Scalar> From<(T, T, Vector<2, T>)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T>)) -> Self {
        Vector::from_array([value.0, value.1, value.2[0], value.2[1]])
    }
}

impl<T: Scalar> From<(T, Vector<2, T>, T)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T>, T)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1], value.2])
    }
}

impl<T: Scalar> From<(T, Vector<3, T>)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T>)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1], value.1[2]])
    }
}

impl<T: Scalar> From<(Vector<2, T>, T, T)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (Vector<2, T>, T, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1, value.2])
    }
}

impl<T: Scalar> From<(Vector<2, T>, Vector<2, T>)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (Vector<2, T>, Vector<2, T>)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1[0], value.1[1]])
    }
}

impl<T: Scalar> From<(Vector<3, T>, T)> for Vector<4, T> {
    #[inline(always)]
    fn from(value: (Vector<3, T>, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.0[2], value.1])
    }
}
