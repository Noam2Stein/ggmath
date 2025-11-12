use crate::{Scalar, SupportedVecLen, VecLen, Vector};

////////////////////////////////////////////////////////////////////////////////
/// Creates a vector2 from 2 scalar values.
///
/// If only a single argument is provided, it is used for all components.
///
/// ## Examples
///
/// ```
/// use ggmath::{Vector, vec2};
///
/// assert_eq(vec2!(1.0, 2.0), Vector::from_array([1.0, 2.0]));
/// assert_eq(vec2!(1.0), Vector::from_array([1.0, 1.0]));
/// ```
#[macro_export]
macro_rules! vec2 {
    ($($arg:expr),* $(,)?) => {
        $crate::Vector::<2, _>::from(($($arg,)*))
    };
}

pub use vec2;

/// Creates a vector3 from 3 scalar values. Also accepts vectors as arguments as
/// long as their lengths sum up to 3.
///
/// If only a single argument is provided, it is used for all components.
///
/// ## Examples
///
/// ```
/// use ggmath::{Vector, vec2, vec3};
///
/// assert_eq(vec3!(1.0, 2.0, 3.0), Vector::from_array([1.0, 2.0, 3.0]));
/// assert_eq(vec3!(vec2!(1.0, 2.0), 3.0), Vector::from_array([1.0, 2.0, 3.0]));
/// assert_eq(vec3!(1.0), Vector::from_array([1.0, 1.0, 1.0]));
/// ```
#[macro_export]
macro_rules! vec3 {
    ($($arg:expr),* $(,)?) => {
        $crate::Vector::<3, _>::from(($($arg,)*))
    };
}

pub use vec3;

/// Creates a vector4 from 4 scalar values. Also accepts vectors as arguments as
/// long as their lengths sum up to 4.
///
/// If only a single argument is provided, it is used for all components.
///
/// ## Examples
///
/// ```
/// use ggmath::{Vector, vec2, vec4};
///
/// assert_eq(vec4!(1.0, 2.0, 3.0, 4.0), Vector::from_array([1.0, 2.0, 3.0, 4.0]));
/// assert_eq(vec4!(1.0, vec2!(2.0, 3.0), 4.0), Vector::from_array([1.0, 2.0, 3.0, 4.0]));
/// assert_eq(vec4!(1.0), Vector::from_array([1.0, 1.0, 1.0, 1.0]));
/// ```
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
