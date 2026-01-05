use crate::vector::{Alignment, Scalar, Vector};

/// Creates a [`Vector`]2 with the given elements.
///
/// If only a single element is given, it will be used for all elements of the
/// [`Vector`].
///
/// ## Examples
///
/// ```
/// use ggmath::{Vec2, vec2};
///
/// let v: Vec2 = vec2!(1.0, 2.0);
/// assert_eq!(v, Vec2::from_array([1.0, 2.0]));
///
/// let v: Vec2 = vec2!(1.0);
/// assert_eq!(v, Vec2::from_array([1.0, 1.0]));
/// ```
#[macro_export]
macro_rules! vec2 {
    ($($arg:expr),*$(,)?) => {
        $crate::vector::Vector::<2, _, _>::from(($($arg,)*))
    };
}

/// Creates a [`Vector`]3 with the given elements. Also accepts [`Vector`]s as
/// arguments, as long as their lengths sum up to 3.
///
/// If only a single element is given, it will be used for all elements of the
/// [`Vector`].
///
/// ## Examples
///
/// ```
/// use ggmath::{Vec3, vec2, vec3};
///
/// let v: Vec3 = vec3!(1.0, 2.0, 3.0);
/// assert_eq!(v, Vec3::from_array([1.0, 2.0, 3.0]));
///
/// let v: Vec3 = vec3!(1.0, vec2!(2.0, 3.0));
/// assert_eq!(v, Vec3::from_array([1.0, 2.0, 3.0]));
///
/// let v: Vec3 = vec3!(1.0);
/// assert_eq!(v, Vec3::from_array([1.0, 1.0, 1.0]));
/// ```
#[macro_export]
macro_rules! vec3 {
    ($($arg:expr),*$(,)?) => {
        $crate::vector::Vector::<3, _, _>::from(($($arg,)*))
    };
}

/// Creates a [`Vector`]4 with the given elements. Also accepts [`Vector`]s as
/// arguments, as long as their lengths sum up to 4.
///
/// If only a single element is given, it will be used for all elements of the
/// [`Vector`].
///
/// ## Examples
///
/// ```
/// use ggmath::{Vec4, vec2, vec4};
///
/// let v: Vec4 = vec4!(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(v, Vec4::from_array([1.0, 2.0, 3.0, 4.0]));
///
/// let v: Vec4 = vec4!(1.0, vec2!(2.0, 3.0), 4.0);
/// assert_eq!(v, Vec4::from_array([1.0, 2.0, 3.0, 4.0]));
///
/// let v: Vec4 = vec4!(1.0);
/// assert_eq!(v, Vec4::from_array([1.0, 1.0, 1.0, 1.0]));
/// ```
#[macro_export]
macro_rules! vec4 {
    ($($arg:expr),*$(,)?) => {
        $crate::vector::Vector::<4, _, _>::from(($($arg,)*))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Vector2 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, A: Alignment> From<(T,)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0)
    }
}

impl<T: Scalar, A: Alignment> From<(T, T)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<2, T, A>,)> for Vector<2, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, A: Alignment> From<(T,)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0, value.0)
    }
}

impl<T: Scalar, A: Alignment> From<(T, T, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T: Scalar, A: Alignment> From<(T, Vector<2, T, A>)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1])
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<2, T, A>, T)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1)
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<3, T, A>,)> for Vector<3, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4 Implementations
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, A: Alignment> From<(T,)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::new(value.0, value.0, value.0, value.0)
    }
}

impl<T: Scalar, A: Alignment> From<(T, T, T, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T: Scalar, A: Alignment> From<(T, T, Vector<2, T, A>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1, value.2[0], value.2[1])
    }
}

impl<T: Scalar, A: Alignment> From<(T, Vector<2, T, A>, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, A>, T)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.2)
    }
}

impl<T: Scalar, A: Alignment> From<(T, Vector<3, T, A>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T, A>)) -> Self {
        Self::new(value.0, value.1[0], value.1[1], value.1[2])
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<2, T, A>, T, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A>, T, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.1, value.2)
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<2, T, A>, Vector<2, T, A>)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<2, T, A>, Vector<2, T, A>)) -> Self {
        Self::new(value.0[0], value.0[1], value.1[0], value.1[1])
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<3, T, A>, T)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<3, T, A>, T)) -> Self {
        Self::new(value.0[0], value.0[1], value.0[2], value.1)
    }
}

impl<T: Scalar, A: Alignment> From<(Vector<4, T, A>,)> for Vector<4, T, A> {
    #[inline(always)]
    fn from(value: (Vector<4, T, A>,)) -> Self {
        value.0
    }
}
